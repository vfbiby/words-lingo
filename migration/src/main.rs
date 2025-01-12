mod word_seed;

use sea_orm_migration::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use crate::word_seed::seed_words;

#[async_std::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Run migrations
    cli::run_cli(migration::Migrator).await;

    // Connect to database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&db_url).await
        .expect("Failed to connect to database");

    // Seed words
    seed_words(&db).await.expect("Failed to seed words");
}
