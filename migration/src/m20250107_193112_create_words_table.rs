use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let parts_of_speech = get_parts_of_speech();
        let part_of_speech_vec = create_part_of_speech_aliases(&parts_of_speech);

        manager
            .create_table(
                Table::create()
                    .table(Word::Table)
                    .if_not_exists()
                    .col(pk_auto(Word::WordId))
                    .col(string(Word::Word))
                    .col(
                        ColumnDef::new(Word::PartOfSpeech)
                            .enumeration(Word::PartOfSpeech, part_of_speech_vec)
                            .not_null(),
                    )
                    .col(string(Word::Definition))
                    .col(string(Word::ExampleSentence))
                    // .index(
                    //     Index::create()
                    //         .name("word_part_of_speech")
                    //         .col(Word::Word)
                    //         .col(Word::PartOfSpeech)
                    //         .unique(),
                    // )
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

fn get_parts_of_speech() -> [&'static str; 13] {
    [
        "noun",
        "verb",
        "vi",
        "adjective",
        "adverb",
        "pronoun",
        "preposition",
        "conjunction",
        "interjection",
        "article",
        "determiner",
        "modal_verb",
        "auxiliary_verb",
    ]
}

fn create_part_of_speech_aliases(parts_of_speech: &[&str]) -> Vec<Alias> {
    parts_of_speech.iter().map(|&pos| Alias::new(pos)).collect()
}

#[derive(DeriveIden)]
pub enum Word {
    Table,
    WordId,
    Word,
    PartOfSpeech,
    Definition,
    ExampleSentence,
}
