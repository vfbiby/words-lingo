use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::OnceCell;

mod word;
mod user;

static SEEDERS_REGISTRY: OnceCell<Mutex<HashMap<&'static str, Arc<dyn Seeder + Send + Sync>>>> = OnceCell::new();

#[async_trait]
pub trait Seeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr>;
}

pub fn register_seeder(name: &'static str, seeder: Arc<dyn Seeder + Send + Sync>) {
    SEEDERS_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .insert(name, seeder);
}

pub fn get_seeders() -> HashMap<&'static str, Arc<dyn Seeder + Send + Sync>> {
    SEEDERS_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap()
        .clone()
}