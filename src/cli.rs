//! Command line interface related logic.

use std::str::FromStr;

use super::util::{current_timestamp_millis, get_today};
use argh::FromArgs;
use chrono::NaiveDate;
use url::Url;

/// iClass API for UCAS.
#[derive(Clone, Debug, FromArgs)]
pub struct Cli {
    /// API root URL, defaulting to UCAS iClass API root
    #[argh(option, short = 'a')]
    pub api_root: Option<Url>,
    /// the subcommand to run
    #[argh(subcommand)]
    pub subcommand: SubCommands,
}

/// The available subcommands.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand)]
pub enum SubCommands {
    /// Login to iClass and save session to a file.
    Login(Login),
    /// List courses in current semester.
    Courses(Courses),
    /// Get schedule for a specific date or week, defaulting to today.
    Schedule(Schedule),
    /// Check-in for a specific schedule by id or uuid, defaulting to current schedule if any.
    CheckIn(CheckIn),
}

/// Login to iClass and save session to a file.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "login")]
pub struct Login {
    /// the username
    #[argh(positional)]
    pub username: String,
    /// the password
    #[argh(positional)]
    pub password: String,
    /// the session file path to save to, defaulting to "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// List courses in current semester.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "courses")]
pub struct Courses {
    /// the session file path to load from, defaulting to "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// Get schedule for a specific date or week, defaulting to today.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "schedule")]
pub struct Schedule {
    /// the date to get schedule for in "YYYY-MM-DD" format, defaulting to today
    #[argh(option, short = 'd', default = "get_today()")]
    pub date: NaiveDate,
    /// show weekly schedule instead of daily schedule
    #[argh(switch, short = 'w')]
    pub weekly: bool,
    /// the session file path to load from, defaulting to "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// Check-in for a specific schedule by id or uuid, defaulting to current schedule if any.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "checkin")]
pub struct CheckIn {
    /// the schedule id or uuid, defaulting to current schedule if any
    #[argh(positional)]
    pub id_or_uuid: Option<String>,
    /// timestamp or offset (prefixed with '+' or '-') to current time in milliseconds, defaulting to +0 (current time)
    #[argh(option, short = 't')]
    pub timestamp_or_offset: Option<String>,
    /// the session file path to load from, defaulting to "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// Timestamp or offset.
pub enum TimestampOrOffset {
    /// A specific timestamp in milliseconds.
    Timestamp(u128),
    /// Positive offset to the current time in milliseconds.
    Plus(u128),
    /// Negative offset to the current time in milliseconds.
    Minus(u128),
}

impl FromStr for TimestampOrOffset {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let prefix = s.chars().next().unwrap_or_default();
        let rest = if prefix == '+' || prefix == '-' {
            &s[1..]
        } else {
            s
        };
        let value: u128 = rest.parse().map_err(|_| ())?;
        match prefix {
            '+' => Ok(Self::Plus(value)),
            '-' => Ok(Self::Minus(value)),
            _ => Ok(Self::Timestamp(value)),
        }
    }
}

impl TimestampOrOffset {
    /// Resolve to a timestamp in milliseconds.
    pub fn resolve(&self) -> u128 {
        match self {
            Self::Timestamp(ts) => *ts,
            Self::Plus(offset) => current_timestamp_millis() + offset,
            Self::Minus(offset) => current_timestamp_millis().saturating_sub(*offset),
        }
    }
}

impl Default for TimestampOrOffset {
    fn default() -> Self {
        Self::Plus(0)
    }
}
