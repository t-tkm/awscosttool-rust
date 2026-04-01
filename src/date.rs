use chrono::{Datelike, Duration, Local, NaiveDate};

/// 集計期間を返す。`Start` は当月1日、`End` は通常は当日（ISO 8601）。
///
/// Cost Explorer の `TimePeriod.End` は排他であり、`Start` より後である必要がある。
/// 月初当日だけ `Start` と「今日」を `End` にすると同一日になり無効になるため、
/// その場合は `End` を月初の翌日に補正する。
pub fn get_date_range() -> (String, String) {
    get_date_range_for(Local::now().date_naive())
}

fn get_date_range_for(today: NaiveDate) -> (String, String) {
    let month_start = today.with_day(1).expect("day 1 always valid");
    let end_date = if today > month_start {
        today
    } else {
        month_start + Duration::days(1)
    };
    (
        month_start.format("%Y-%m-%d").to_string(),
        end_date.format("%Y-%m-%d").to_string(),
    )
}

/// 表示用ラベルを返す。
/// Cost Explorer の End は「その日の 0:00」を指すため、表示上は end - 1日。
pub fn format_display_dates(start: &str, end: &str) -> (String, String) {
    let start_date = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d")
        .expect("start date format is always valid");
    let end_date = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d")
        .expect("end date format is always valid");
    let display_end = end_date - Duration::days(1);
    (
        start_date.format("%m/%d").to_string(),
        display_end.format("%m/%d").to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_display_dates() {
        let (s, e) = format_display_dates("2026-03-01", "2026-03-31");
        assert_eq!(s, "03/01");
        assert_eq!(e, "03/30");
    }

    #[test]
    fn test_format_display_dates_first_day() {
        // 月初に実行した場合: end = start なので display_end = 前月末日
        let (s, e) = format_display_dates("2026-03-01", "2026-03-01");
        assert_eq!(s, "03/01");
        assert_eq!(e, "02/28");
    }

    #[test]
    fn test_get_date_range_start_is_first_of_month() {
        let (start, _end) = get_date_range();
        assert!(
            start.ends_with("-01"),
            "start should be first of month: {start}"
        );
    }

    #[test]
    fn test_get_date_range_first_of_month_end_is_next_day() {
        let (start, end) = super::get_date_range_for(
            NaiveDate::from_ymd_opt(2026, 3, 1).expect("valid date"),
        );
        assert_eq!(start, "2026-03-01");
        assert_eq!(end, "2026-03-02");
    }

    #[test]
    fn test_get_date_range_mid_month_unchanged() {
        let (start, end) = super::get_date_range_for(
            NaiveDate::from_ymd_opt(2026, 3, 15).expect("valid date"),
        );
        assert_eq!(start, "2026-03-01");
        assert_eq!(end, "2026-03-15");
    }
}
