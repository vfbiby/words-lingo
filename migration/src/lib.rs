pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250107_193112_create_words_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250107_193112_create_words_table::Migration),
        ]
    }
}
