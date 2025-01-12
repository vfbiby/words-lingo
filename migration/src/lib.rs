pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250107_193112_create_words_table;
mod m20250107_193113_create_users_table;
mod m20250112_172322_create_user_word_progress_table;
mod m20250112_185317_create_content_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250107_193112_create_words_table::Migration),
            Box::new(m20250107_193113_create_users_table::Migration),
            Box::new(m20250112_172322_create_user_word_progress_table::Migration),
            Box::new(m20250112_185317_create_content_table::Migration),
        ]
    }
}
