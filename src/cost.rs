use anyhow::{Context, Result};
use aws_sdk_costexplorer::types::{
    DateInterval, Dimension, DimensionValues, Expression, GroupDefinition, GroupDefinitionType,
};
use aws_sdk_costexplorer::Client as CeClient;
use aws_sdk_sts::Client as StsClient;

use crate::config::*;

/// クレジット適用モード
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CreditMode {
    /// クレジット適用後（フィルタなし）
    AfterCredit,
    /// クレジット適用前（Credit レコードを除外）
    BeforeCredit,
}

impl CreditMode {
    pub fn label(&self) -> &'static str {
        match self {
            CreditMode::AfterCredit => "後",
            CreditMode::BeforeCredit => "前",
        }
    }
}

/// サービスごとのコスト
#[derive(Debug, Clone)]
pub struct ServiceCost {
    pub service_name: String,
    pub amount: f64,
}

/// 1回分のコストレポート
#[derive(Debug)]
pub struct CostReport {
    pub account_id: String,
    pub start_label: String,
    pub end_label: String,
    pub credit_mode: CreditMode,
    pub total_cost: f64,
    pub services: Vec<ServiceCost>,
}

/// AWS STS からアカウント ID を取得する。
pub async fn get_account_id(client: &StsClient) -> Result<String> {
    let resp = client
        .get_caller_identity()
        .send()
        .await
        .context("Failed to call STS GetCallerIdentity")?;
    resp.account
        .ok_or_else(|| anyhow::anyhow!("Account ID not found in STS response"))
}

/// Cost Explorer API を呼び出してコストレポートを取得する。
pub async fn fetch_cost_report(
    client: &CeClient,
    start: &str,
    end: &str,
    start_label: &str,
    end_label: &str,
    account_id: &str,
    credit_mode: CreditMode,
) -> Result<CostReport> {
    let period = DateInterval::builder()
        .start(start)
        .end(end)
        .build()
        .context("Failed to build DateInterval")?;

    let group = GroupDefinition::builder()
        .r#type(GroupDefinitionType::Dimension)
        .key(SERVICE_GROUP_DIMENSION)
        .build();

    let mut req = client
        .get_cost_and_usage()
        .time_period(period)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .metrics(COST_METRIC)
        .group_by(group);

    if credit_mode == CreditMode::BeforeCredit {
        let dim_values = DimensionValues::builder()
            .key(Dimension::RecordType)
            .values(CREDIT_RECORD_TYPE)
            .build();
        let not_expr = Expression::builder()
            .not(Expression::builder().dimensions(dim_values).build())
            .build();
        req = req.filter(not_expr);
    }

    let resp = req
        .send()
        .await
        .context("Failed to call Cost Explorer GetCostAndUsage")?;

    let result = resp
        .results_by_time
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| anyhow::anyhow!("No results returned from Cost Explorer"))?;

    // サービスごとのコストを収集
    let services: Vec<ServiceCost> = result
        .groups
        .unwrap_or_default()
        .into_iter()
        .filter_map(|g| {
            let name = g.keys?.into_iter().next()?;
            let amount: f64 = g
                .metrics?
                .get(COST_METRIC)?
                .amount
                .as_deref()
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0);
            Some(ServiceCost {
                service_name: name,
                amount,
            })
        })
        .collect();

    // 合計コストを計算
    let total_cost = if let Some(totals) = result.total {
        totals
            .get(COST_METRIC)
            .and_then(|m| m.amount.as_deref())
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
    } else {
        // Total が空の場合は Groups から合算（負値は 0 に丸める）
        services.iter().map(|s| s.amount.max(0.0)).sum()
    };

    Ok(CostReport {
        account_id: account_id.to_string(),
        start_label: start_label.to_string(),
        end_label: end_label.to_string(),
        credit_mode,
        total_cost,
        services,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_mode_label() {
        assert_eq!(CreditMode::AfterCredit.label(), "後");
        assert_eq!(CreditMode::BeforeCredit.label(), "前");
    }

    #[test]
    fn test_total_from_groups() {
        // Total が空の場合、Groups から合算されることを確認するロジックテスト
        let services = vec![
            ServiceCost {
                service_name: "EC2".into(),
                amount: 10.5,
            },
            ServiceCost {
                service_name: "S3".into(),
                amount: -1.0, // 負値
            },
            ServiceCost {
                service_name: "RDS".into(),
                amount: 5.0,
            },
        ];
        let total: f64 = services.iter().map(|s| s.amount.max(0.0)).sum();
        assert!((total - 15.5).abs() < 1e-9);
    }
}
