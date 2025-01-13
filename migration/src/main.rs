use sea_orm_migration::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use std::env;
use seed::seed_words;

#[async_std::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Get the command from command line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
    println!("Command: {}", command);

    match command {
        // Seed words
        "seed" => {
            // Connect to database
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let db: DatabaseConnection = Database::connect(&db_url).await
                .expect("Failed to connect to database");

            // Seed words
            seed_words(&db).await.expect("Failed to seed words");
        },
        // Default behavior (run migrations)
        _ => {
            cli::run_cli(migration::Migrator).await;
        }
    }
}