# Tech Stack Decisions - aws-cost-report

## 言語・エディション
| 項目 | 決定 | 理由 |
|---|---|---|
| 言語 | Rust 2021 edition | 要件通り |
| 非同期ランタイム | `tokio` | AWS SDK for Rust の必須依存 |

## 主要依存クレート

| クレート | バージョン指定 | 用途 |
|---|---|---|
| `aws-config` | `1` | AWS 認証情報・設定の読み込み |
| `aws-sdk-costexplorer` | `1` | Cost Explorer API クライアント |
| `aws-sdk-sts` | `1` | STS GetCallerIdentity |
| `tokio` | `1` (features = ["full"]) | 非同期ランタイム |
| `env_logger` | `0.11` | シンプルなテキストログ（RUST_LOG 制御） |
| `log` | `0.4` | ログマクロ（info!, debug!, error!） |
| `chrono` | `0.4` | 日付計算（当月1日・当日・前日） |
| `anyhow` | `1` | エラー伝播の簡略化 |

## 採用しないクレート（理由）
| クレート | 不採用理由 |
|---|---|
| `reqwest` / `hyper` | Teams 通知不要のため HTTP クライアント不要 |
| `serde_json` | Teams ペイロード生成不要のため不要 |
| `cargo-lambda` | Lambda 対応不要 |
| `tracing` | シンプルな `env_logger` で十分 |

## モジュール構成

```
src/
  main.rs        - エントリポイント、main() / run() 関数
  config.rs      - 定数定義（リージョン、メトリクス名等）
  date.rs        - 日付計算ロジック
  cost.rs        - Cost Explorer API 呼び出し・レスポンス解析
  report.rs      - レポートフォーマット・標準出力表示
```

## ビルド設定
- `Cargo.lock` をバージョン管理に含める（依存バージョン固定）
- リリースビルド: `cargo build --release`
- テスト: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- フォーマット: `cargo fmt --check`
