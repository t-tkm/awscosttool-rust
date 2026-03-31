# Integration Test Instructions

## 概要
このツールは単一バイナリで AWS API を直接呼び出す構成のため、統合テストは実 AWS 環境（または LocalStack）に対して実行する。

## シナリオ 1: 実 AWS 環境での動作確認

### 前提条件
- AWS 認証情報が設定済み
- `ce:GetCostAndUsage` と `sts:GetCallerIdentity` の IAM 権限あり

### 実行手順
```bash
# リリースビルド
cargo build --release

# 実行
./target/release/aws-cost-report
```

### 期待される出力
```
------------------------------------------------------
AWSアカウント <アカウントID>
MM/01～MM/DD のクレジット適用後費用は、XX.XX USD です。
- Amazon EC2: XX.XX USD
...
------------------------------------------------------

------------------------------------------------------
AWSアカウント <アカウントID>
MM/01～MM/DD のクレジット適用前費用は、XX.XX USD です。
...
------------------------------------------------------
```

### 確認ポイント
- [ ] 終了コードが 0 であること（`echo $?`）
- [ ] クレジット適用後・前の両レポートが出力されること
- [ ] アカウント ID が正しく表示されること
- [ ] 0.01 USD 未満のサービスが表示されないこと

## シナリオ 2: エラーハンドリング確認

### 認証情報なしでの実行
```bash
AWS_ACCESS_KEY_ID=invalid AWS_SECRET_ACCESS_KEY=invalid \
  ./target/release/aws-cost-report
```

### 期待される動作
- エラーメッセージが stderr に出力される
- 終了コードが非ゼロ（`echo $?` → `1`）

## シナリオ 3: ログレベル確認
```bash
RUST_LOG=debug ./target/release/aws-cost-report
```
- 0.01 USD 未満のサービスが DEBUG ログに記録されることを確認

## LocalStack を使ったオフラインテスト（オプション）
LocalStack を使用する場合は `AWS_ENDPOINT_URL` を設定して実行できるが、
Cost Explorer は LocalStack Pro が必要なため、基本的には実 AWS 環境での確認を推奨。
