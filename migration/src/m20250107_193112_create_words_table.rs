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
                    .col(
                        ColumnDef::new(Word::PartOfSpeech)
                            .enumeration(
                                Word::PartOfSpeech,
                                vec![
                                    Alias::new("noun"),
                                    Alias::new("verb"),
                                    Alias::new("adjective"),
                                    Alias::new("adverb"),
                                    Alias::new("pronoun"),
                                    Alias::new("preposition"),
                                    Alias::new("conjunction"),
                                    Alias::new("interjection"),
                                    Alias::new("article"),
                                    Alias::new("determiner"),
                                    Alias::new("modal_verb"),
                                    Alias::new("auxiliary_verb"),
                                ],
                            )
                            .not_null(),
                    )
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
