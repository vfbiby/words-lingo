use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Word::Table)
                    .if_not_exists()
                    .col(pk_auto(Word::WordId))
                    .col(string(Word::Word))
                    .col(string(Word::PartOfSpeech))
                    .col(string(Word::Definition))
                    .col(string(Word::ExampleSentence))
                    .index(
                        Index::create()
                            .name("word_part_of_speech")
                            .col(Word::Word)
                            .col(Word::PartOfSpeech)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Word::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Word {
    Table,
    WordId,
    Word,
    PartOfSpeech,
    Definition,
    ExampleSentence,
}
