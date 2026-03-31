# Unit Test Execution

## テスト対象モジュール

| モジュール | テスト内容 |
|---|---|
| `src/date.rs` | 日付フォーマット、月初判定、月初実行時の前日計算 |
| `src/cost.rs` | CreditMode ラベル、負値の 0 丸め合算ロジック |
| `src/report.rs` | 閾値フィルタ、閾値ちょうどの包含、空サービスリスト |

## ユニットテスト実行

### 全テスト実行
```bash
cargo test
```

### 詳細出力付き
```bash
cargo test -- --nocapture
```

### 特定モジュールのみ
```bash
cargo test date::tests
cargo test cost::tests
cargo test report::tests
```

## 期待される結果
```
running 8 tests
test date::tests::test_format_display_dates ... ok
test date::tests::test_format_display_dates_first_day ... ok
test date::tests::test_get_date_range_start_is_first_of_month ... ok
test cost::tests::test_credit_mode_label ... ok
test cost::tests::test_total_from_groups ... ok
test report::tests::test_format_filters_below_threshold ... ok
test report::tests::test_format_includes_threshold_exactly ... ok
test report::tests::test_format_empty_services ... ok

test result: ok. 8 passed; 0 failed; 0 ignored
```

## テスト失敗時の対処
1. `cargo test -- --nocapture` で詳細ログを確認
2. 失敗したテスト名から対象モジュールを特定
3. コードを修正して再実行
