# Requirements Verification Questions

以下の質問に回答してください。各質問の `[Answer]:` タグの後に選択肢の文字を記入してください。
選択肢に合うものがない場合は最後の選択肢（Other）を選び、説明を追記してください。

---

## Question 1
このツールの主な実行形態はどれですか？

A) CLIバイナリとして実行（ローカルPC・サーバー上で直接実行）
B) AWS Lambda 関数として実行（Python版と同様）
C) 両方対応（CLIとしてもLambdaとしても動作）
D) Other (please describe after [Answer]: tag below)

[Answer]: A

---

## Question 2
Teams への通知機能は必要ですか？

A) はい、Python版と同じ機能（Adaptive Card + text フォールバック）が必要
B) はい、ただしシンプルなテキスト通知のみで十分
C) いいえ、標準出力への表示のみで十分
D) Other (please describe after [Answer]: tag below)

[Answer]: C

---

## Question 3
AWS Secrets Manager からの Webhook URL 取得機能は必要ですか？

A) はい、Python版と同様に TEAMS_SECRET_ARN 環境変数経由での取得が必要
B) いいえ、環境変数 TEAMS_WEBHOOK_URL からの直接取得のみで十分
C) Other (please describe after [Answer]: tag below)

[Answer]: B

---

## Question 4
対象とする AWS リージョンはどれですか？

A) us-east-1 固定（Python版と同じ）
B) 環境変数で設定可能にする
C) CLIオプションで指定可能にする
D) Other (please describe after [Answer]: tag below)

[Answer]: A

---

## Question 5
クレジット適用前後の両レポートを出力しますか？

A) はい、Python版と同様にクレジット適用後・適用前の両方を出力
B) いいえ、クレジット適用後のみ
C) CLIオプションで選択可能にする
D) Other (please describe after [Answer]: tag below)

[Answer]: A

---

## Question 6
Rust のエディションとターゲットについて教えてください。

A) Rust 2021 edition、ネイティブバイナリ（x86_64-unknown-linux-gnu など）
B) Rust 2021 edition、Lambda 向け（cargo-lambda 使用）
C) Rust 2021 edition、両方（ネイティブ + Lambda）
D) Other (please describe after [Answer]: tag below)

[Answer]: A

---

## Question 7
ログ出力の形式はどうしますか？

A) Python版と同様のシンプルなテキストログ（INFO/DEBUG/ERROR レベル）
B) 構造化ログ（JSON形式）
C) Other (please describe after [Answer]: tag below)

[Answer]: A

---

## Question 8: Security Extensions
Should security extension rules be enforced for this project?

A) Yes — enforce all SECURITY rules as blocking constraints (recommended for production-grade applications)
B) No — skip all SECURITY rules (suitable for PoCs, prototypes, and experimental projects)
X) Other (please describe after [Answer]: tag below)

[Answer]: B
