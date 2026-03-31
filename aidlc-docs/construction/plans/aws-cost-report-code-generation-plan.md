# Code Generation Plan - aws-cost-report

## Unit Context
- **Unit**: aws-cost-report（単一 Rust バイナリ）
- **Project Type**: Greenfield
- **Workspace Root**: /workspace
- **Target Directory**: workspace root（`src/`, `Cargo.toml` 等）

## Steps

- [x] Step 1: Cargo プロジェクト初期化
  - `Cargo.toml` を作成（依存クレート: aws-config, aws-sdk-costexplorer, aws-sdk-sts, tokio, env_logger, log, chrono, anyhow）

- [x] Step 2: `src/config.rs` 生成
  - 定数定義（REGION, GRANULARITY, COST_METRIC, SERVICE_DIMENSION, RECORD_TYPE_DIMENSION, CREDIT_RECORD_TYPE, MIN_BILLING_THRESHOLD）

- [x] Step 3: `src/date.rs` 生成
  - `get_date_range()` → `(String, String)` 当月1日・当日
  - `format_display_dates()` → `(String, String)` 表示用ラベル（MM/DD 形式、終了は -1日）
  - ユニットテスト

- [x] Step 4: `src/cost.rs` 生成
  - `CreditMode` 列挙型
  - `ServiceCost` 構造体
  - `CostReport` 構造体
  - `fetch_cost_report()` 非同期関数（CE API 呼び出し・レスポンス解析）
  - `get_account_id()` 非同期関数（STS API 呼び出し）
  - ユニットテスト（コスト集計ロジック）

- [x] Step 5: `src/report.rs` 生成
  - `format_service_costs()` → `Vec<String>`（0.01 USD 閾値フィルタ）
  - `print_report()` → 標準出力表示
  - ユニットテスト（フォーマットロジック）

- [x] Step 6: `src/main.rs` 生成
  - `run()` 非同期関数（メイン処理フロー）
  - `main()` エントリポイント（tokio::main、エラー時非ゼロ終了）

- [x] Step 7: ドキュメント生成
  - `aidlc-docs/construction/aws-cost-report/code/code-summary.md`（生成ファイル一覧・使用方法）
