//! Utility functions.

use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use serde::de::{self, Deserialize, Deserializer};
use std::time::{SystemTime, UNIX_EPOCH};

/// UTC+8 timezone for China Standard Time.
pub const CST_TIMEZONE: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();

/// Deserialize a string (1/0) to a boolean.
///
/// # Errors
///
/// If the string is not "1" or "0".
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

/// Deserialize a string to an i8.
///
/// # Errors
///
/// If the string cannot be parsed to i8.
pub fn deserialize_str_to_int<'de, D>(deserializer: D) -> Result<i8, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse()
        .map_err(|e| de::Error::custom(format!("invalid integer string: {s}, error: {e}")))
}

/// Deserialize an optional string to an optional i8.
///
/// # Errors
///
/// If the string cannot be parsed to i8.
pub fn deserialize_opt_str_to_int<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    s.map(|str_val| {
        str_val.parse().map_err(|e| {
            de::Error::custom(format!("invalid integer string: {str_val}, error: {e}"))
        })
    })
    .transpose()
}

/// Deserialize a string (YYYY-MM-DD HH:MM:SS) to a [`DateTime`] in China Standard Time.
///
/// # Errors
///
/// If the string is not in the correct format ("YYYY-MM-DD HH:MM:SS") or is ambiguous in CST.
pub fn deserialize_str_to_datetime<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
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
///
/// # Errors
///
/// If the string is not in the correct format ("YYYYMMDD").
pub fn deserialize_str_to_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let naive_date = NaiveDate::parse_from_str(s, "%Y%m%d").map_err(|e| {
        de::Error::custom(format!(
            "invalid date string: {s}, expected YYmmdd, error: {e}"
        ))
    })?;
    Ok(naive_date)
}

/// Deserialize a string (YYYY-MM-DD) to a [`NaiveDate`].
///
/// # Errors
///
/// If the string is not in the correct format ("YYYY-MM-DD").
pub fn deserialize_str_to_date_hyphen<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let naive_date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| {
        de::Error::custom(format!(
            "invalid date string: {s}, expected YY-mm-dd, error: {e}"
        ))
    })?;
    Ok(naive_date)
}

/// Format a [`NaiveDate`] to a string (YYYYMMDD).
#[must_use]
pub fn format_date_to_str(date: &NaiveDate) -> String {
    date.format("%Y%m%d").to_string()
}

/// Format a [`DateTime`] to a string (YYYY-MM-DD HH:MM:SS) without timezone.
#[must_use]
pub fn format_datetime_to_str(datetime: &DateTime<FixedOffset>) -> String {
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Get current timestamp in milliseconds since UNIX epoch.
///
/// # Panics
///
/// If system time is before UNIX epoch.
#[must_use]
pub fn current_timestamp_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

/// Get today.
#[must_use]
pub fn get_today() -> NaiveDate {
    Utc::now().with_timezone(&CST_TIMEZONE).date_naive()
}
