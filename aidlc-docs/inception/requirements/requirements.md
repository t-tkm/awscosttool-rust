# Requirements Document

## Intent Analysis

- **User Request**: Python製 AWS コストレポートツールを Rust の CLI ツールとして移植する
- **Request Type**: New Project（Migration / Port）
- **Scope Estimate**: Single Component（単一バイナリ）
- **Complexity Estimate**: Moderate（AWS SDK Rust、非同期処理、HTTP クライアント）

---

## Functional Requirements

### FR-01: AWS Cost Explorer によるコスト取得
- AWS Cost Explorer API (`GetCostAndUsage`) を呼び出し、当月1日から実行日までのコストを取得する（`TimePeriod` の詳細は FR-06）
- 集計粒度: MONTHLY
- メトリクス: AmortizedCost
- グループ化: SERVICE ディメンション

### FR-02: クレジット適用後レポートの生成
- クレジットを含む（RECORD_TYPE フィルタなし）コストを取得する
- サービスごとの費用を一覧表示する
- 合計費用を表示する

### FR-03: クレジット適用前レポートの生成
- クレジットを除外（RECORD_TYPE = Credit を Not フィルタ）したコストを取得する
- サービスごとの費用を一覧表示する
- 合計費用を表示する

### FR-04: AWS アカウント ID の表示
- AWS STS `GetCallerIdentity` でアカウント ID を取得し、レポートのタイトルに含める

### FR-05: 標準出力へのレポート表示
- クレジット適用後・適用前の順に、区切り線付きで標準出力に表示する
- 表示形式は Python 版と同等（`------...` 区切り、`- サービス名: X.XX USD` 形式）
- 0.01 USD 未満のサービスは表示しない（DEBUG ログに記録）

### FR-06: 集計期間の自動計算
- 開始日: 当月1日（`TimePeriod.Start`）
- API 用終了日（`TimePeriod.End`）: 通常は実行日（当日）。Cost Explorer は `End` を**排他**とし、`Start` より後である必要があるため、**実行日が当月1日のときだけ** `End` を「月初の翌日」に補正する（`Start` と同一日を避ける）
- 表示ラベル: `End` は当日 0:00 前を指すため、画面上の終了日は「API 用 `End` の日付 − 1 日」として扱う（例: `End=2026-04-02` なら表示は 04/01 まで）

---

## Non-Functional Requirements

### NFR-01: 実行形態
- ネイティブ CLI バイナリとしてビルド・実行できること
- Rust 2021 edition を使用すること

### NFR-02: リージョン
- AWS リージョンは `us-east-1` 固定

### NFR-03: ログ出力
- `tracing` または `env_logger` 等を使用したシンプルなテキストログ
- INFO / DEBUG / ERROR レベルをサポート

### NFR-04: エラーハンドリング
- AWS API 呼び出し失敗時は適切なエラーメッセージを出力して終了する
- パニックではなく `Result` 型でエラーを伝播させる

### NFR-05: 依存関係
- AWS SDK for Rust（`aws-sdk-costexplorer`, `aws-sdk-sts`）を使用
- 非同期ランタイムは `tokio` を使用
- Cargo.lock をコミット対象とする（依存バージョン固定）

---

## Out of Scope

- Teams Webhook 通知機能（不要）
- AWS Secrets Manager 連携（不要）
- AWS Lambda 対応（不要）
- 構造化ログ（JSON形式）（不要）
- リージョンの動的設定（不要）

---

## Extension Configuration

| Extension | Enabled | Decided At |
|---|---|---|
| security/baseline | No | Requirements Analysis |
