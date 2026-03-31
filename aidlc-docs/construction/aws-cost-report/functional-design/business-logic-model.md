# Business Logic Model

## メイン処理フロー

```
main()
  |
  +-- 1. get_date_range()
  |       -> start = 当月1日 (YYYY-MM-DD)
  |       -> end   = 当日    (YYYY-MM-DD)
  |
  +-- 2. get_account_id(sts_client)
  |       -> STS GetCallerIdentity -> account_id: String
  |
  +-- 3. fetch_cost_report(ce_client, period, CreditMode::AfterCredit)
  |       -> CostReport（クレジット適用後）
  |
  +-- 4. print_report(report)
  |       -> 標準出力に表示
  |
  +-- 5. fetch_cost_report(ce_client, period, CreditMode::BeforeCredit)
  |       -> CostReport（クレジット適用前）
  |
  +-- 6. print_report(report)
          -> 標準出力に表示
```

## fetch_cost_report ロジック

```
fetch_cost_report(client, period, credit_mode):
  1. build_filter(credit_mode)
       AfterCredit  -> フィルタなし
       BeforeCredit -> Not { Dimensions { RECORD_TYPE = ["Credit"] } }

  2. GetCostAndUsage API 呼び出し
       TimePeriod  = period
       Granularity = MONTHLY
       Metrics     = ["AmortizedCost"]
       GroupBy     = [{ Type: DIMENSION, Key: SERVICE }]
       Filter      = (上記)

  3. レスポンス解析
       ResultsByTime[0].Groups から ServiceCost リストを構築
       amount = Groups[i].Metrics["AmortizedCost"].Amount (f64)

  4. 合計コスト計算
       Total が存在する場合 -> Total["AmortizedCost"].Amount
       存在しない場合       -> Groups の amount を合算（負値は 0 に丸める）

  5. CostReport を返す
```

## print_report ロジック

```
print_report(report):
  1. タイトル行を生成
       "AWSアカウント {account_id}"
       "{start_label}～{end_label}のクレジット適用{後|前}費用は、{total:.2f} USD です。"

  2. 区切り線を出力
       "------------------------------------------------------"

  3. タイトルを出力

  4. サービス一覧を出力
       amount >= 0.01 USD のサービスのみ
       "- {service_name}: {amount:.2f} USD"
       該当なし -> "サービスごとの費用データはありません。"

  5. 区切り線を出力
```

## 表示ラベル計算

```
start_label = 当月1日.format("%m/%d")          // 例: "03/01"
end_label   = (当日 - 1日).format("%m/%d")     // 例: "03/30"
              ※ Cost Explorer の End は当日 0:00 を指すため表示上は前日
```
