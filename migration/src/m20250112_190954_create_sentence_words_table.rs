use sea_orm_migration::{prelude::*, schema::*};
use crate::m20250107_193112_create_words_table::Word;
use crate::m20250112_190406_create_sentences_table::Sentence;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SentenceWord::Table)
                    .if_not_exists()
                    .col(pk_auto(SentenceWord::SentenceWordId))
                    .col(integer(SentenceWord::SentenceId).not_null())
                    .col(integer(SentenceWord::WordId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sentence_word-sentence_id")
                            .from(SentenceWord::Table, SentenceWord::SentenceId)
                            .to(Sentence::Table, Sentence::SentenceId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-sentence_word-word_id")
                            .from(SentenceWord::Table, SentenceWord::WordId)
                            .to(Word::Table, Word::WordId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SentenceWord::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SentenceWord {
    Table,
    SentenceWordId,
    SentenceId,
    WordId,
}
