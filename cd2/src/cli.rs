use anyhow::Result;
use std::{path::{Path, PathBuf}, process::exit};
use clap::{
    command,
    Command,
    Arg
};
use lazy_static::lazy_static;
use crate::{
    PROJECT_DATA_DIR,
    db::{DB_URL, connect_to_db, upsert_dir},
    guess::guess_dir
};

const COMMAND_EXIT_CODE: i32 = 42;

lazy_static! {
    static ref OUTPUT_DIR_FILE_PATH: PathBuf = PROJECT_DATA_DIR
        .join("output")
        .with_extension("txt");
}

pub async fn run() -> Result<()> {
    let command = define_command();

    // Get args supplied
    let matches = command.get_matches();

    // Input dir path
    let dir_path = Path::new(
        matches
            .get_one::<String>("dir")
            .unwrap()
    );

    // Connec to database
    let pool = connect_to_db(&DB_URL).await?;

    // Guess the most probablr dir
    let dir = guess_dir(
            dir_path,
            &pool
        )
        .await
        .unwrap();
    
    // Update or insert the path in database
    upsert_dir(&dir.path, &pool).await?;

    // Write output dir to a file
    std::fs::write(
        OUTPUT_DIR_FILE_PATH.as_path(), 
        &dir.path.to_str().unwrap()
    )?;
    
    // Exit with a special code
    exit(COMMAND_EXIT_CODE);
}

fn define_command() -> Command {
    // Make an empty command
    let mut command = command!()
        .name("cd2")
        .bin_name("cd2")
        .about("Smart Directory Navigation");

    // Dir argument
    let dir_arg = Arg::new("dir")
        .required(true)
        .value_name("DIR")
        .help("Directory to navigate to");

    // Add arguments
    command = command.args([
        dir_arg
    ]);

    command
}
