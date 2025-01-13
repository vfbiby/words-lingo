use clap::{Arg, Command};
use sea_orm::{Database, DatabaseConnection};
use dotenv::dotenv;
use std::collections::HashMap;

mod seeds;

#[async_std::main]
async fn main() {
    // Load .env file
    dotenv().ok();

    // Parse command line arguments
    let matches = Command::new("seed")
        .version("1.0")
        .author("Your Name")
        .about("Database seeder")
        .arg(
            Arg::new("seeder")
                .short('s')
                .long("seeder")
                .value_name("SEEDER_NAME")
                .help("Name of the seeder to run")
                .num_args(1),
        )
        .get_matches();

    // Connect to database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&db_url).await
        .expect("Failed to connect to database");

    // Get all seeders
    let seeders: HashMap<&str, Box<dyn seeds::Seeder + Send + Sync>> = seeds::get_seeders();

    if let Some(seeder_name) = matches.get_one::<String>("seeder") {
        // Run specific seeder
        if let Some(seeder) = seeders.get(seeder_name.as_str()) {
            seeder.seed(&db).await.expect(&format!("Failed to run seeder: {}", seeder_name));
        } else {
            eprintln!("Seeder '{}' not found", seeder_name);
            std::process::exit(1);
        }
    } else {
        // Run all seeders
        for (name, seeder) in seeders {
            println!("Running seeder: {}", name);
            seeder.seed(&db).await.expect(&format!("Failed to run seeder: {}", name));
        }
    }
}