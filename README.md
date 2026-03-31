# aws-cost-report

[![CI](https://github.com/<your-username>/aws-cost-report/actions/workflows/ci.yml/badge.svg)](https://github.com/<your-username>/aws-cost-report/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

AWS の当月コストをサービス別に標準出力へ表示する Rust 製 CLI ツールです。
クレジット適用後・適用前の両レポートを出力します。

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

## 必要な環境

- Rust stable（1.75 以上推奨）
- AWS CLI v2
- AWS 認証情報（IAM Identity Center / SSO 推奨）

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

## AWS 認証情報の準備

長期的なアクセスキー（`aws_access_key_id` / `aws_secret_access_key`）の使用は AWS の公式ベストプラクティスで非推奨です。
**IAM Identity Center（SSO）による一時認証情報の使用を推奨します。**

### 方法 1: IAM Identity Center（SSO）を使う（推奨）

#### 1. AWS CLI v2 のインストール

```bash
# macOS（Homebrew）
brew install awscli

# バージョン確認
aws --version
```

#### 2. SSO プロファイルの設定（初回のみ）

```bash
aws configure sso --profile billing-user
```

対話形式で以下を入力します：

```
SSO session name (Recommended): my-sso
SSO start URL [None]: https://<your-domain>.awsapps.com/start
SSO region [None]: ap-northeast-1
SSO registration scopes [sso:account:access]: （Enter）
Default client Region [ap-northeast-1]: （Enter）
CLI default output format [None]: json
```

設定後、`~/.aws/config` に以下のようなブロックが追記されます：

```ini
[profile billing-user]
sso_session = my-sso
sso_account_id = <account_id>
sso_role_name = <permission_set>
region = ap-northeast-1
output = json

[sso-session my-sso]
sso_start_url = https://d-xxxxxxxxxx.awsapps.com/start/
sso_region = ap-northeast-1
sso_registration_scopes = sso:account:access
```

#### 3. SSO ログイン

```bash
aws sso login --profile billing-user
```

ブラウザが開くので IAM Identity Center でログインします。

#### 4. 接続確認

```bash
AWS_PROFILE=billing-user aws sts get-caller-identity
```

#### 5. ツールの実行

```bash
AWS_PROFILE=billing-user ./target/release/aws-cost-report
```

毎回 `AWS_PROFILE` を指定する代わりに、シェルに環境変数を設定しておくと便利です：

```bash
export AWS_PROFILE=billing-user
./target/release/aws-cost-report
```

> 一時認証情報の有効期間は許可セットのセッション設定に依存します（デフォルト 1 時間、最大 12 時間）。
> 期限切れ後は `aws sso login` を再実行してください。

---

### 方法 2: 環境変数で認証情報を直接渡す

IAM ユーザーのアクセスキーや、他ツールで取得した一時認証情報を使う場合：

```bash
export AWS_ACCESS_KEY_ID=AKIA...
export AWS_SECRET_ACCESS_KEY=xxxx
export AWS_SESSION_TOKEN=xxxx   # 一時認証情報の場合のみ
export AWS_DEFAULT_REGION=us-east-1

./target/release/aws-cost-report
```

## インストール

### バイナリをダウンロード（推奨）

[Releases](https://github.com/t-tkm/awscosttool-rust/releases) から使用環境に合ったバイナリをダウンロードしてください。

| ファイル名 | 対象環境 |
|---|---|
| `aws-cost-report-*-aarch64-apple-darwin.tar.gz` | M1/M2/M3 Mac |
| `aws-cost-report-*-x86_64-apple-darwin.tar.gz` | Intel Mac |
| `aws-cost-report-*-x86_64-unknown-linux-gnu.tar.gz` | Linux x86_64 |
| `aws-cost-report-*-aarch64-unknown-linux-gnu.tar.gz` | Linux ARM64 |
| `aws-cost-report-*-x86_64-pc-windows-msvc.zip` | Windows x86_64 |

```bash
# 例: M1 Mac の場合
tar xzf aws-cost-report-v0.1.0-aarch64-apple-darwin.tar.gz
./aws-cost-report
```

### ソースからビルド

```bash
cargo build --release
```

## 実行

```bash
./target/release/aws-cost-report
```

## ログレベルの制御

`RUST_LOG` 環境変数でログの詳細度を変更できます。

```bash
# 通常実行（INFO レベル）
RUST_LOG=info ./target/release/aws-cost-report

# デバッグ情報も表示（0.01 USD 未満の除外サービスも記録）
RUST_LOG=debug ./target/release/aws-cost-report
```

## 仕様

| 項目 | 内容 |
|---|---|
| 集計期間 | 当月1日〜当日 |
| メトリクス | AmortizedCost |
| グループ化 | サービス別 |
| リージョン | us-east-1 固定 |
| 最小表示閾値 | 0.01 USD（未満は非表示） |

## テスト

```bash
cargo test
```

## 開発

```bash
# Lint
rustup component add clippy
cargo clippy -- -D warnings

# フォーマット
cargo fmt
```

## ライセンス

[MIT](LICENSE)

## 付録: クロスコンパイル

`cargo build --release` は実行しているマシンのアーキテクチャ向けのバイナリのみを生成します。

他プラットフォーム向けのバイナリを手元で作るにはクロスコンパイルが必要で、ツールチェーンの追加インストールが必要になります。

```bash
# 例: M1 Mac から Intel Mac 向けをビルド
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin
```

Linux や Windows 向けはさらに複雑（リンカの追加が必要）なため、GitHub Actions の release ワークフローに任せることにする。タグを push すれば全プラットフォーム分が自動でビルドされて Releases に添付される。

```bash
git tag v0.1.0
git push origin v0.1.0
```
