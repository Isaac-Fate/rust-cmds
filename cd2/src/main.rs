use anyhow::Result;
use cd2::db::{DB_URL, connect_to_db};
use cd2::cli::run;

#[tokio::main]
async fn main() -> Result<()> {
    let _pool = connect_to_db(&DB_URL).await?;

    run().await?;

    Ok(())
}
