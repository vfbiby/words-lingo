use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250107_193113_create_users_table::User;
use crate::m20250107_193112_create_words_table::Word;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserWordProgress::Table)
                    .if_not_exists()
                    .col(pk_auto(UserWordProgress::ProgressId))
                    .col(integer(UserWordProgress::UserId).not_null())
                    .col(integer(UserWordProgress::WordId).not_null())
                    .col(
                        ColumnDef::new(UserWordProgress::ProficiencyLevel)
                            .enumeration(
                                UserWordProgress::ProficiencyLevel,
                                vec![
                                    Alias::new("not_started"),
                                    Alias::new("learning"),
                                    Alias::new("mastered"),
                                ],
                            )
                            .default(Value::String(Some(Box::new("not_started".to_owned())))),
                    )
                    .col(ColumnDef::new(UserWordProgress::LastReviewed).timestamp_with_time_zone())
                    .col(integer(UserWordProgress::ReviewCount).default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(UserWordProgress::Table, UserWordProgress::UserId)
                            .to(User::Table, User::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_word_id")
                            .from(UserWordProgress::Table, UserWordProgress::WordId)
                            .to(Word::Table, Word::WordId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserWordProgress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserWordProgress {
    Table,
    ProgressId,
    UserId,
    WordId,
    ProficiencyLevel,
    LastReviewed,
    ReviewCount,
}
