//! Command line interface related logic.

use super::util::get_today;
use argh::FromArgs;
use chrono::NaiveDate;

/// iClass API for UCAS.
#[derive(Clone, Debug, FromArgs)]
pub struct Cli {
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
    /// Get schedule for a specific date or week.
    Schedule(Schedule),
    /// Check-in for a specific schedule by id or uuid.
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
    /// the session file path to save to, default is "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// List courses in current semester.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "courses")]
pub struct Courses {
    /// the session file path to load from, default is "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// Get schedule for a specific date or week.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "schedule")]
pub struct Schedule {
    /// the date to get schedule for in "YYYY-MM-DD" format, default is today
    #[argh(
        option,
        short = 'd',
        default = "get_today()"
    )]
    pub date: NaiveDate,
    /// show weekly schedule instead of daily schedule
    #[argh(switch, short = 'w')]
    pub weekly: bool,
    /// the session file path to load from, default is "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}

/// Check-in for a specific schedule by id or uuid.
#[derive(Clone, Debug, FromArgs)]
#[argh(subcommand, name = "checkin")]
pub struct CheckIn {
    /// the schedule id or uuid, default is to check-in current schedule if any
    #[argh(positional)]
    pub id_or_uuid: Option<String>,
    /// the session file path to load from, default is "session.json"
    #[argh(option, short = 's', default = "String::from(\"session.json\")")]
    pub session_file: String,
}
