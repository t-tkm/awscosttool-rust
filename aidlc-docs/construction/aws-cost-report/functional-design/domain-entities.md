# Domain Entities

## CostPeriod
集計期間を表す値オブジェクト（Cost Explorer `TimePeriod` にそのまま渡す）。

```
CostPeriod {
    start: String  // "YYYY-MM-DD" 当月1日
    end:   String  // "YYYY-MM-DD" 通常は実行日。実行日が当月1日のときのみ「月初の翌日」
}
```

表示上の「〜まで」は `end` の暦日より 1 日前（`format_display_dates`）。

## ServiceCost
サービスごとのコストを表す値オブジェクト。

```
ServiceCost {
    service_name: String
    amount:       f64     // USD
}
```

## CostReport
1回分のレポート（クレジット適用後 or 前）を表す集約。

```
CostReport {
    account_id:    String
    period_label:  String        // "MM/DD～MM/DD"
    credit_mode:   CreditMode    // AfterCredit | BeforeCredit
    total_cost:    f64           // USD
    services:      Vec<ServiceCost>
}
```

## CreditMode
クレジット適用の有無を表す列挙型。

```
CreditMode {
    AfterCredit,   // クレジット適用後（フィルタなし）
    BeforeCredit,  // クレジット適用前（Credit レコードを除外）
}
```
