use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use async_trait::async_trait;
use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
use words_lingo::entity::word::{ActiveModel, Model};
use crate::seeds::{Seeder, register_seeder};
use std::sync::Arc;
use ctor::ctor;
use csv::Reader;
use std::fs::File;
use std::path::Path;

// 配置常量
const CSV_PATH: &str = "word_translation.csv";
const MAX_RECORDS: usize = 100;

pub struct WordSeeder;

#[ctor]
fn register_word_seeder() {
    register_seeder("word", Arc::new(WordSeeder) as Arc<dyn Seeder + Send + Sync>);
}

#[async_trait]
impl Seeder for WordSeeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        seed_words(db).await?;
        seed_words_from_csv(db).await
    }
}

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
        let active_model: ActiveModel = word.into();
        active_model.insert(db).await?;
    }

    Ok(())
}

pub async fn seed_words_from_csv(db: &DatabaseConnection) -> Result<(), DbErr> {
    let path = Path::new(CSV_PATH);
    let file = File::open(path).map_err(|e| DbErr::Custom(format!("无法打开CSV文件: {}, 错误: {}", CSV_PATH, e)))?;
    let mut rdr = Reader::from_reader(file);
    
    let mut word_id = 4; // 从4开始，避免与现有数据冲突
    let mut count = 0;

    for result in rdr.records() {
        if count >= MAX_RECORDS {
            break;
        }

        let record = result.map_err(|e| DbErr::Custom(format!("CSV解析错误: {}", e)))?;
        let word = &record[0];
        let translation = &record[1];

        let (part_of_speech, definition) = parse_translation(translation)?;

        let model = Model {
            word_id,
            word: word.to_string(),
            part_of_speech,
            definition,
            example_sentence: String::new(), // 暂时留空
        };

        let active_model: ActiveModel = model.into();
        active_model.insert(db).await?;

        word_id += 1;
        count += 1;
    }

    Ok(())
}

fn parse_translation(translation: &str) -> Result<(PartOfSpeech, String), DbErr> {
    // 支持的词性标记
    let pos_markers = [
        ("n.", PartOfSpeech::Noun),
        ("v.", PartOfSpeech::Verb),
        ("adj.", PartOfSpeech::Adjective),
    ];

    let mut pos = PartOfSpeech::Noun; // 默认名词
    let mut definitions = Vec::new();
    let remaining = translation;

    // 查找第一个词性标记
    if let Some((marker, found_pos, _)) = pos_markers.iter()
        .filter_map(|(m, p)| remaining.find(m).map(|idx| (m, p, idx)))
        .min_by_key(|&(_, _, idx)| idx)
    {
        pos = found_pos.clone();
        // 提取定义部分
        let def_part = &remaining[marker.len()..];
        // 处理多个定义
        let defs: Vec<&str> = def_part.split(',').collect();
        definitions.extend(defs.iter().map(|s| {
            let prefix = match pos {
                PartOfSpeech::Noun => "[名]",
                PartOfSpeech::Verb => "[动]",
                PartOfSpeech::Adjective => "[形]",
                _ => "[未分类]",
            };
            format!("{} {}", prefix, s.trim())
        }));
    } else {
        // 没有找到词性标记，整个作为定义
        definitions.push(format!("[未分类] {}", remaining));
    }

    Ok((pos, definitions.join("；")))
}
