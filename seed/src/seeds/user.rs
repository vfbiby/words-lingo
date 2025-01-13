use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use words_lingo::entity::user::{Model, ActiveModel};
use chrono::{Utc, DateTime};

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