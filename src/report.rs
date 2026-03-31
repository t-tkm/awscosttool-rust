use crate::config::MIN_BILLING_THRESHOLD;
use crate::cost::CostReport;

/// サービスコストを表示用文字列リストに変換する。
/// MIN_BILLING_THRESHOLD 未満のサービスは除外し、DEBUG ログに記録する。
pub fn format_service_costs(report: &CostReport) -> Vec<String> {
    report
        .services
        .iter()
        .filter_map(|s| {
            if s.amount >= MIN_BILLING_THRESHOLD {
                Some(format!("- {}: {:.2} USD", s.service_name, s.amount))
            } else {
                log::debug!(
                    "Excluded negligible cost: {} ({:.5})",
                    s.service_name,
                    s.amount
                );
                None
            }
        })
        .collect()
}

/// レポートを標準出力に表示する。
pub fn print_report(report: &CostReport) {
    let credit_label = report.credit_mode.label();
    let title_account = format!("AWSアカウント {}", report.account_id);
    let title_cost = format!(
        "{}～{}のクレジット適用{}費用は、{:.2} USD です。",
        report.start_label, report.end_label, credit_label, report.total_cost
    );

    let formatted = format_service_costs(report);

    println!("------------------------------------------------------");
    println!("{}", title_account);
    println!("{}", title_cost);
    if formatted.is_empty() {
        println!("サービスごとの費用データはありません。");
    } else {
        for line in &formatted {
            println!("{}", line);
        }
    }
    println!("------------------------------------------------------");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cost::{CreditMode, ServiceCost};

    fn make_report(services: Vec<ServiceCost>) -> CostReport {
        CostReport {
            account_id: "123456789012".into(),
            start_label: "03/01".into(),
            end_label: "03/30".into(),
            credit_mode: CreditMode::AfterCredit,
            total_cost: services.iter().map(|s| s.amount).sum(),
            services,
        }
    }

    #[test]
    fn test_format_filters_below_threshold() {
        let report = make_report(vec![
            ServiceCost {
                service_name: "EC2".into(),
                amount: 10.0,
            },
            ServiceCost {
                service_name: "Tiny".into(),
                amount: 0.005,
            },
        ]);
        let lines = format_service_costs(&report);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("EC2"));
    }

    #[test]
    fn test_format_includes_threshold_exactly() {
        let report = make_report(vec![ServiceCost {
            service_name: "S3".into(),
            amount: 0.01,
        }]);
        let lines = format_service_costs(&report);
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("0.01 USD"));
    }

    #[test]
    fn test_format_empty_services() {
        let report = make_report(vec![]);
        let lines = format_service_costs(&report);
        assert!(lines.is_empty());
    }
}
