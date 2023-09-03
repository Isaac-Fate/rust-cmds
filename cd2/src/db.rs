use std::path::{Path, PathBuf};
use chrono::Local;
use anyhow::Result;
use sqlx::{
    Sqlite,
    SqlitePool, 
    migrate::{MigrateDatabase, Migrator}
};
use lazy_static::lazy_static;
use crate::{
    PROJECT_DATA_DIR,
    dir::Directory
};

lazy_static! {
    pub static ref DB_URL: String = {
        let db_url = format!(
            "sqlite://{}",
            PROJECT_DATA_DIR
                .join("cd2")
                .with_extension("db")
                .to_str()
                .unwrap()
        );

        db_url
    };

    static ref MIGRATIONS_DIR: PathBuf = PROJECT_DATA_DIR.join("migrations");
}

const DIRS_TABLE_NAME: &'static str = "dirs";

pub async fn connect_to_db(url: &str) -> Result<SqlitePool> {
    // The database does not exist
    // or there is an error
    if !Sqlite::database_exists(url)
        .await
        .unwrap_or(false) {
        Sqlite::create_database(url).await?;
    }

    // Connection pool
    let pool = SqlitePool::connect(url).await?;

    // Execute migration scripts
    // to create the table
    Migrator::new(MIGRATIONS_DIR.as_path())
        .await?
        .run(&pool)
        .await?;

    Ok(pool)
}

pub async fn find_all_dirs(pool: &SqlitePool) -> Result<Vec<Directory>> {
    let dirs = sqlx::query_as::<_, Directory>(
            format!(
                r#"
                SELECT * FROM {}
                "#,
                DIRS_TABLE_NAME
            )
            .as_str()
        )
        .fetch_all(pool)
        .await?;
  
    Ok(dirs)
}

pub async fn insert_dir<P>(path: P, pool: &SqlitePool) -> Result<()> 
where P: AsRef<Path> 
{
    sqlx::query(
        format!(
            r#"
            INSERT INTO {} (path, last_visit_time)
            VALUES ($1, $2)
            "#,
            DIRS_TABLE_NAME
        )
        .as_str()
        
    )
    .bind(
        path
            .as_ref()
            .to_str()
            .unwrap()
    )
    .bind(Local::now().naive_local())
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn upsert_dir<P>(path: P, pool: &SqlitePool) -> Result<()>
where P: AsRef<Path>
{
    match sqlx::query(
        format!(
            r#"
            UPDATE {}
            SET last_visit_time = $1
            WHERE path = $2
            "#,
            DIRS_TABLE_NAME
        )
        .as_str()
    )
    .bind(Local::now().naive_local())
    .bind(
        path
            .as_ref()
            .to_str()
            .unwrap()
    )
    .execute(pool)
    .await?
    .rows_affected() {
        0 => {
            insert_dir(path, pool).await?;
            
        },
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx::sqlite::SqlitePool;
    use home::home_dir;
    use lazy_static::lazy_static;

    use super::{
        PROJECT_DATA_DIR,
        connect_to_db,
        find_all_dirs,
        insert_dir,
        upsert_dir
    };

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
    async fn find_all_dirs_from_database() -> Result<()> {
        // Connect to test database
        let pool = connect_to_test_db().await?;

        let dirs = find_all_dirs(&pool).await?;
        println!("{:#?}", dirs);

        Ok(())
    }

    #[tokio::test]
    async fn insert_home_dir() -> Result<()> {
        // Connect to test database
        let pool = connect_to_test_db().await?;

        insert_dir(
            home_dir().unwrap(), 
            &pool
        )
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn upsert_dirs() -> Result<()> {
        // Connect to test database
        let pool = connect_to_test_db().await?;

        // Upsert home dir
        upsert_dir(
            home_dir().unwrap(), 
            &pool
        )
        .await?;

        // Upsert documents dir
        upsert_dir(
            home_dir()
                .unwrap()
                .join("Documents"), 
            &pool
        )
        .await?;

        Ok(())
    }
}
