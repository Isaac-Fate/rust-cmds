use std::path::{Path, PathBuf};
use chrono::{NaiveDateTime, Local};
use sqlx::{FromRow, Row, sqlite::SqliteRow};

// #[derive(FromRow)]
#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub last_visit_time: NaiveDateTime
}

impl FromRow<'_, SqliteRow> for Directory {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            path: PathBuf::from(row.try_get::<&str, _>("path")?),
            last_visit_time: row.try_get("last_visit_time")?
        })
    }
}

impl Directory {
    pub fn new<P>(path: P) -> Self 
    where P: AsRef<Path> {
        Directory { 
            path: path
                .as_ref()
                .to_owned(), 
            last_visit_time: Local::now().naive_local()
        }
    }

    pub fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}
