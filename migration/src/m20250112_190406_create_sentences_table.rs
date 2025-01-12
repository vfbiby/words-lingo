use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250112_185317_create_content_table::Content;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sentence::Table)
                    .if_not_exists()
                    .col(pk_auto(Sentence::SentenceId))
                    .col(integer(Sentence::ContentId).not_null())
                    .col(integer(Sentence::SentenceOrder).not_null())
                    .col(text(Sentence::SentenceText).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sentence-content_id")
                            .from(Sentence::Table, Sentence::ContentId)
                            .to(Content::Table, Content::ContentId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sentence::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Sentence {
    Table,
    SentenceId,
    ContentId,
    SentenceOrder,
    SentenceText,
}
