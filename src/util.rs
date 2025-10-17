//! Utility functions.

use serde::de::{self, Deserialize, Deserializer};

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
