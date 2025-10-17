#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use ucas_iclass::{IClass, IClassError};

#[compio::main]
async fn main() -> Result<(), IClassError> {
    // Get username and password from command line argument #1 and #2
    let mut args = std::env::args();
    args.next(); // Skip program name
    let username = args
        .next()
        .expect("Please provide username as the first argument");
    let password = args
        .next()
        .expect("Please provide password as the second argument");

    let mut iclass = IClass::new();
    iclass.login(&username, &password).await?;
    let login_result = iclass.login_result.as_ref().unwrap();
    println!(
        "Logged in as {} (student_no={}, id={})",
        login_result.real_name, login_result.student_no, login_result.id
    );

    let courses = iclass.query_courses().await?;
    println!("\nCourses in current semester:");
    for course in &courses {
        println!("  {course}");
    }

    let daily_schedule = iclass.query_daily_schedule("20251017").await?;
    println!("\nDaily schedule on 2025-10-17:");
    for schedule in &daily_schedule {
        println!("  {schedule}");
    }

    let weekly_schedule = iclass.query_weekly_schedule("20251017").await?;
    println!("\nWeekly schedule for week of 2025-10-17:");
    for daily_schedule in &weekly_schedule {
        println!("{daily_schedule}");
    }

    Ok(())
}
