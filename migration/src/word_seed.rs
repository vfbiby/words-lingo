use migration::DbErr;
use migration::sea_orm::DatabaseConnection;
use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
// use sea_orm::database::db_connection::DatabaseConnection;
use words_lingo::entity::word::Model;

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
        word.insert(db).await?;
    }

    Ok(())
}