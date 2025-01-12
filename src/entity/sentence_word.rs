//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sentence_word")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub sentence_word_id: i32,
    pub sentence_id: i32,
    pub word_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sentence::Entity",
        from = "Column::SentenceId",
        to = "super::sentence::Column::SentenceId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Sentence,
    #[sea_orm(
        belongs_to = "super::word::Entity",
        from = "Column::WordId",
        to = "super::word::Column::WordId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Word,
}

impl Related<super::sentence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sentence.def()
    }
}

impl Related<super::word::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Word.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}