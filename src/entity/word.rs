//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use super::sea_orm_active_enums::PartOfSpeech;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "word")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub word_id: i32,
    pub word: String,
    pub part_of_speech: PartOfSpeech,
    pub definition: String,
    pub example_sentence: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::sentence_word::Entity")]
    SentenceWord,
    #[sea_orm(has_many = "super::user_word_progress::Entity")]
    UserWordProgress,
}

impl Related<super::sentence_word::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SentenceWord.def()
    }
}

impl Related<super::user_word_progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserWordProgress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}