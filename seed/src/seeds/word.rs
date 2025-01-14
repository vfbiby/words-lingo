use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use async_trait::async_trait;
use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
use words_lingo::entity::word::{ActiveModel, Model};
use crate::seeds::{Seeder, register_seeder};
use std::sync::Arc;
use ctor::ctor;

pub struct WordSeeder;

// 使用ctor在模块加载时注册
#[ctor]
fn register_word_seeder() {
    register_seeder("word", Arc::new(WordSeeder) as Arc<dyn Seeder + Send + Sync>);
}

#[async_trait]
impl Seeder for WordSeeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        seed_words(db).await
    }
}

pub async fn seed_words(db: &DatabaseConnection) -> Result<(), DbErr> {
    let words = vec![
        Model {
            word_id: 1,
            word: "apple".to_string(),
            part_of_speech: PartOfSpeech::Noun,
            definition: "a round fruit with red, yellow, or green skin".to_string(),
            example_sentence: "I ate an apple for breakfast".to_string(),
        },
        Model {
            word_id: 2,
            word: "run".to_string(),
            part_of_speech: PartOfSpeech::Verb,
            definition: "move quickly on foot".to_string(),
            example_sentence: "He runs every morning".to_string(),
        },
        Model {
            word_id: 3,
            word: "beautiful".to_string(),
            part_of_speech: PartOfSpeech::Adjective,
            definition: "pleasing the senses or mind aesthetically".to_string(),
            example_sentence: "She has a beautiful smile".to_string(),
        },
    ];

    for word in words {
        let active_model: ActiveModel = word.into();
        active_model.insert(db).await?;
    }

    Ok(())
}
