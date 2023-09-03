use std::path::Path;
use sqlx::sqlite::SqlitePool;
use crate::{
    dir::Directory,
    db::find_all_dirs
};

/// Guess the most probable dir
/// from the all the visited dirs in database.
pub async fn guess_dir<P>(path: P, pool: &SqlitePool) -> Option<Directory>
where P: AsRef<Path>
{
    // Return the dir immediately
    // if it exists
    if path.as_ref().is_dir() {
        return Some(Directory::new(
            // Convert to absolute path
            std::fs::canonicalize(path).unwrap()
        ));
    }

    // Guess the most probable dir
    // from the all the visited dirs in database
    find_all_dirs(pool)
        .await
        .unwrap()
        .into_iter()
        .max_by(|dir1, dir2| {
            let dir_name = path
                .as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            strsim::jaro_winkler(
                dir_name,
                &dir1.name()
            )
            .total_cmp(
                &strsim::jaro(
                    dir_name,
                    &dir2.name()
                )
            )
        })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx::sqlite::SqlitePool;
    use lazy_static::lazy_static;

    use crate::{
        PROJECT_DATA_DIR,
        db::connect_to_db,
    };
    use super::guess_dir;

    lazy_static! {
        static ref TEST_DB_URL: String = {
            let db_url = format!(
                "sqlite://{}",
                PROJECT_DATA_DIR
                    .join("test-cd2")
                    .with_extension("db")
                    .to_str()
                    .unwrap()
            );
    
            db_url
        };
    }

    async fn connect_to_test_db() -> Result<SqlitePool> {
        connect_to_db(&TEST_DB_URL).await
    }

    #[tokio::test]
    async fn guess_the_most_probable_dir() -> Result<()> {
        // Connect to test database
        let pool = connect_to_test_db().await?;

        let dir = guess_dir("doc", &pool).await;
        println!("{:?}", dir);

        Ok(())
    }
}