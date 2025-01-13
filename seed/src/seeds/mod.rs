use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

mod word;

#[async_trait]
pub trait Seeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr>;
}

pub struct WordSeeder;

#[async_trait]
impl Seeder for WordSeeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        word::seed_words(db).await
    }
}

pub fn get_seeders() -> HashMap<&'static str, Box<dyn Seeder + Send + Sync>> {
    let mut seeders = HashMap::new();
    seeders.insert("word", Box::new(WordSeeder) as Box<dyn Seeder + Send + Sync>);
    seeders
}