pub mod db;
mod dir;
mod guess;
pub mod cli;

use std::path::PathBuf;
use lazy_static::lazy_static;
use home::home_dir;

lazy_static! {
    /// Data directory of this project.
    pub static ref PROJECT_DATA_DIR: PathBuf = {
        // Data directory
        let dir = home_dir()
            .unwrap()
            .join(".cd2");

        // Create data directory if it does not exist
        if !dir.is_dir() {
            std::fs::create_dir_all(&dir)
                .expect(format!("Failed to create {:?}", &dir).as_str())
        }

        dir
    };
}