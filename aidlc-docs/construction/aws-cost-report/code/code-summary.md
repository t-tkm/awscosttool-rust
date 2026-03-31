# Code Summary - aws-cost-report

## 生成ファイル一覧

| ファイル | 役割 |
|---|---|
| `Cargo.toml` | プロジェクト定義・依存クレート |
| `src/main.rs` | エントリポイント、`run()` メイン処理フロー |
| `src/config.rs` | 定数定義 |
| `src/date.rs` | 日付計算（集計期間・表示ラベル） |
| `src/cost.rs` | AWS API 呼び出し、データ構造定義 |
| `src/report.rs` | レポートフォーマット・標準出力表示 |

## ビルド・実行方法

```bash
# ビルド
cargo build --release

# 実行（AWS 認証情報が設定済みであること）
./target/release/aws-cost-report

# ログレベル制御
RUST_LOG=debug ./target/release/aws-cost-report

# テスト
cargo test

# Lint
cargo clippy -- -D warnings

# フォーマット確認
cargo fmt --check
```

## 必要な IAM 権限

```json
{
  "Effect": "Allow",
  "Action": [
    "ce:GetCostAndUsage",
    "sts:GetCallerIdentity"
  ],
  "Resource": "*"
}
```

## 出力例

```
------------------------------------------------------
AWSアカウント 123456789012
03/01～03/30のクレジット適用後費用は、45.23 USD です。
- Amazon EC2: 30.10 USD
- Amazon S3: 10.05 USD
- AWS Lambda: 5.08 USD
------------------------------------------------------

------------------------------------------------------
AWSアカウント 123456789012
03/01～03/30のクレジット適用前費用は、50.00 USD です。
- Amazon EC2: 35.00 USD
- Amazon S3: 10.00 USD
- AWS Lambda: 5.00 USD
------------------------------------------------------
```
