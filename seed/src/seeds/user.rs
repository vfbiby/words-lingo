use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use words_lingo::entity::user::{ActiveModel, Model};
use chrono::{DateTime, Utc};
use async_trait::async_trait;
use crate::seeds::{Seeder, register_seeder};
use std::sync::Arc;
use ctor::ctor;

pub struct UserSeeder;

// 使用ctor在模块加载时注册
#[ctor]
fn register_user_seeder() {
    register_seeder("user", Arc::new(UserSeeder) as Arc<dyn Seeder + Send + Sync>);
}

#[async_trait]
impl Seeder for UserSeeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        seed_users(db).await
    }
}

pub async fn seed_users(db: &DatabaseConnection) -> Result<(), DbErr> {
    let users = vec![
        Model {
            user_id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Some(DateTime::<Utc>::from(Utc::now())),
        },
        Model {
            user_id: 2,
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Some(DateTime::<Utc>::from(Utc::now())),
        },
    ];

    for user in users {
        let active_model: ActiveModel = user.into();
        active_model.insert(db).await?;
    }

    Ok(())
}
