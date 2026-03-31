mod config;
mod cost;
mod date;
mod report;

use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_costexplorer::config::Region;

use cost::{fetch_cost_report, get_account_id, CreditMode};
use report::print_report;

async fn run() -> Result<()> {
    // AWS SDK 設定（us-east-1 固定）
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(config::REGION_NAME))
        .load()
        .await;

    let ce_client = aws_sdk_costexplorer::Client::new(&sdk_config);
    let sts_client = aws_sdk_sts::Client::new(&sdk_config);

    // アカウント ID 取得
    let account_id = get_account_id(&sts_client).await?;
    log::info!("AWS Account ID: {}", account_id);

    // 集計期間
    let (start, end) = date::get_date_range();
    let (start_label, end_label) = date::format_display_dates(&start, &end);

    // クレジット適用後レポート
    let report_after = fetch_cost_report(
        &ce_client,
        &start,
        &end,
        &start_label,
        &end_label,
        &account_id,
        CreditMode::AfterCredit,
    )
    .await?;
    print_report(&report_after);

    // クレジット適用前レポート
    let report_before = fetch_cost_report(
        &ce_client,
        &start,
        &end,
        &start_label,
        &end_label,
        &account_id,
        CreditMode::BeforeCredit,
    )
    .await?;
    print_report(&report_before);

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = run().await {
        log::error!("Error: {:#}", e);
        std::process::exit(1);
    }
}
