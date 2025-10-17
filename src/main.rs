#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use ucas_iclass::{Course, IClass, IClassError};

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
        let Course { id, name, classroom_name, teacher_name, .. } = course;
        println!("  {name} ({id}) - {teacher_name} @ {classroom_name}");
    }

    Ok(())
}
