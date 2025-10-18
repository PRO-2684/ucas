//! Utility functions.

use chrono::{FixedOffset, DateTime, NaiveDateTime, NaiveDate};
use serde::de::{self, Deserialize, Deserializer};
use std::time::{SystemTime, UNIX_EPOCH};

/// UTC+8 timezone for China Standard Time.
pub const CST_TIMEZONE: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();

/// Deserialize a string (1/0) to a boolean.
pub fn deserialize_str_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        _ => Err(de::Error::custom(format!("invalid boolean string: {s}"))),
    }
}

/// Deserialize a string (YYYY-MM-DD HH:MM:SS) to a [`DateTime`] in China Standard Time.
pub fn deserialize_str_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let naive_dt = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| de::Error::custom(format!("invalid datetime string: {s}, error: {e}")))?;
    let datetime = naive_dt
        .and_local_timezone(CST_TIMEZONE)
        .single()
        .ok_or_else(|| de::Error::custom(format!("ambiguous datetime string: {s}")))?;
    Ok(datetime)
}

/// Deserialize a string (YYYYMMDD) to a [`NaiveDate`].
pub fn deserialize_str_to_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let naive_date = NaiveDate::parse_from_str(s, "%Y%m%d")
        .map_err(|e| de::Error::custom(format!("invalid date string: {s}, error: {e}")))?;
    Ok(naive_date)
}

/// Format a [`NaiveDate`] to a string (YYYYMMDD).
pub fn format_date_to_str(date: &NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

/// Format a [`DateTime`] to a string (YYYY-MM-DD HH:MM:SS) without timezone.
pub fn format_datetime_to_str(datetime: &DateTime<FixedOffset>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Get current timestamp in milliseconds since UNIX epoch.
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}
