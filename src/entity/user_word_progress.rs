//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.4

use super::sea_orm_active_enums::ProficiencyLevel;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_word_progress")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub progress_id: i32,
    pub user_id: i32,
    pub word_id: i32,
    pub proficiency_level: Option<ProficiencyLevel>,
    pub last_reviewed: Option<DateTimeUtc>,
    pub review_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::word::Entity",
        from = "Column::WordId",
        to = "super::word::Column::WordId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Word,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::word::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Word.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
