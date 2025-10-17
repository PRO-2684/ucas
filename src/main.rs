#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use ucas_iclass::{IClass, IClassError, Semester};

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

    let semesters = iclass.query_semester().await?;
    for semester in semesters {
        let Semester {
            code,
            name,
            begin_date,
            end_date,
            year_status,
        } = semester;
        println!("{name} ({code})");
        println!("  Begin: {begin_date}");
        println!("  End: {end_date}");
        println!("  Current: {year_status}");
    }

    Ok(())
}
