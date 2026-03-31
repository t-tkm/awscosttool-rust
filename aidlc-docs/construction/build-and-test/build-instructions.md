# Build Instructions

## Prerequisites
- **Build Tool**: Cargo（Rust 2021 edition）
- **Rust バージョン**: stable（1.75 以上推奨）
- **AWS 認証情報**: 環境変数・プロファイル・IAM ロールのいずれかで設定済みであること
- **必要な IAM 権限**: `ce:GetCostAndUsage`, `sts:GetCallerIdentity`

## Build Steps

### 1. Rust のインストール（未インストールの場合）
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. 依存クレートの取得
```bash
cargo fetch
```

### 3. デバッグビルド
```bash
cargo build
```

### 4. リリースビルド（本番用）
```bash
cargo build --release
```

### 5. ビルド成功の確認
- **成功時の出力例**: `Finished release [optimized] target(s) in XX.XXs`
- **生成バイナリ**: `target/release/aws-cost-report`

## Lint・フォーマット確認

```bash
# Lint（警告をエラーとして扱う）
cargo clippy -- -D warnings

# フォーマット確認
cargo fmt --check

# フォーマット適用
cargo fmt
```

## Troubleshooting

### AWS SDK のコンパイルが遅い
- 初回ビルドは AWS SDK のコンパイルに数分かかる場合がある（正常）
- 2回目以降はキャッシュが効くため高速

### 認証情報エラー（実行時）
```
Error: Failed to call STS GetCallerIdentity
```
- `aws configure` または環境変数 `AWS_ACCESS_KEY_ID` / `AWS_SECRET_ACCESS_KEY` / `AWS_SESSION_TOKEN` を確認
- IAM ロールを使用する場合はロールに `sts:GetCallerIdentity` 権限があることを確認
