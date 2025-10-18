#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use ucas_iclass::{IClass, cli::{Cli, SubCommands, Login, Courses, Schedule, CheckIn}};
use anyhow::{bail, Result};

#[compio::main]
async fn main() -> Result<()> {
    let cli: Cli = argh::from_env();
    let mut iclass = IClass::new();

    match cli.subcommand {
        SubCommands::Login(Login { username, password, session_file }) => {
            iclass.login(&username, &password).await?;
            iclass.save_session_to_file(&session_file)?;
            let user_session = iclass.user_session.as_ref().unwrap();
            println!(
                "Logged in as {} (student_no={}, id={}), session saved to {}",
                user_session.real_name, user_session.student_no, user_session.id, session_file
            );
        }
        SubCommands::Courses(Courses { session_file }) => {
            iclass.restore_session_from_file(&session_file)?;
            let courses = iclass.query_courses().await?;
            println!("Courses in current semester:");
            for course in &courses {
                println!("  {course}");
            }
        }
        SubCommands::Schedule(Schedule { date, weekly, session_file }) => {
            iclass.restore_session_from_file(&session_file)?;
            if weekly {
                let weekly_schedule = iclass.query_weekly_schedule(&date).await?;
                println!("Weekly schedule for week of {date}:");
                for daily_schedule in &weekly_schedule {
                    println!("{daily_schedule}");
                }
            } else {
                let daily_schedule = iclass.query_daily_schedule(&date).await?;
                println!("Daily schedule on {date}:");
                for schedule in &daily_schedule {
                    println!("  {schedule}");
                }
            }
        }
        SubCommands::CheckIn(CheckIn { id_or_uuid, session_file }) => {
            iclass.restore_session_from_file(&session_file)?;
            // id is all numeric, uuid is all hexadecimal and 32 characters long
            let (type_, check_in_result) = if id_or_uuid.len() == 32 && id_or_uuid.chars().all(|c| c.is_ascii_hexdigit()) {
                ("id", iclass.check_in_by_uuid(&id_or_uuid).await?)
            } else if id_or_uuid.chars().all(char::is_numeric) {
                ("uuid", iclass.check_in_by_id(&id_or_uuid).await?)
            } else {
                bail!("Invalid id or uuid format: {id_or_uuid}");
            };
            println!("Check-in by {type_} for schedule {id_or_uuid}: {check_in_result}");
        }
    }

    Ok(())
}
