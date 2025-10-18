#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use ucas_iclass::{IClass, IClassError, util::CST_TIMEZONE};

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
    let user_session = iclass.user_session.as_ref().unwrap();
    println!(
        "Logged in as {} (student_no={}, id={})",
        user_session.real_name, user_session.student_no, user_session.id
    );

    let courses = iclass.query_courses().await?;
    println!("\nCourses in current semester:");
    for course in &courses {
        println!("  {course}");
    }

    let today = chrono::Utc::now().with_timezone(&CST_TIMEZONE).date_naive();

    let daily_schedule = iclass.query_daily_schedule(&today).await?;
    println!("\nDaily schedule on {today}:");
    for schedule in &daily_schedule {
        println!("  {schedule}");
    }

    let weekly_schedule = iclass.query_weekly_schedule(&today).await?;
    println!("\nWeekly schedule for week of {today}:");
    for daily_schedule in &weekly_schedule {
        println!("{daily_schedule}");
    }

    if let Some(id_or_uuid) = args.next() {
        // Treat it as id if it is all digits, otherwise treat it as uuid
        if id_or_uuid.chars().all(char::is_numeric) {
            let check_in_result = iclass.check_in_by_id(&id_or_uuid).await?;
            println!("\nCheck-in result for schedule {id_or_uuid} (by id): {check_in_result}");
        } else {
            let check_in_result = iclass.check_in_by_uuid(&id_or_uuid).await?;
            println!("\nCheck-in result for schedule {id_or_uuid} (by uuid): {check_in_result}");
        }
    }

    Ok(())
}
