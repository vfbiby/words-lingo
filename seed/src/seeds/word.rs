use crate::seeds::{register_seeder, Seeder};
use async_trait::async_trait;
use csv::Reader;
use ctor::ctor;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
use words_lingo::entity::word::{ActiveModel, Model};

// 配置常量
const CSV_PATH: &str = "word_translation.csv";
const MAX_RECORDS: usize = 1000;

pub struct WordSeeder;

#[ctor]
fn register_word_seeder() {
    register_seeder(
        "word",
        Arc::new(WordSeeder) as Arc<dyn Seeder + Send + Sync>,
    );
}

#[async_trait]
impl Seeder for WordSeeder {
    async fn seed(&self, db: &DatabaseConnection) -> Result<(), DbErr> {
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
    let file = File::open(path)
        .map_err(|e| DbErr::Custom(format!("无法打开CSV文件: {}, 错误: {}", CSV_PATH, e)))?;
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

        let entries = parse_translation(translation)?;
        for (part_of_speech, definition) in entries {
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
    }

    Ok(())
}

fn parse_translation(translation: &str) -> Result<Vec<(PartOfSpeech, String)>, DbErr> {
    // 支持的词性标记
    let pos_markers = [
        ("n.", PartOfSpeech::Noun),
        ("vt.", PartOfSpeech::Vt),
        ("vi.", PartOfSpeech::Vi),
        ("v.", PartOfSpeech::Verb),
        ("adj.", PartOfSpeech::Adjective),
    ];

    let mut entries = Vec::new();
    let mut remaining = translation;

    // 查找所有词性标记
    while !remaining.is_empty() {
        if let Some((marker, found_pos, idx)) = pos_markers
            .iter()
            .filter_map(|(m, p)| remaining.find(m).map(|i| (m, p, i)))
            .min_by_key(|&(_, _, i)| i)
        {
            // 提取当前词性部分
            let pos = found_pos.clone();
            let def_part = if let Some(next_marker) = pos_markers
                .iter()
                .filter_map(|(m, _)| remaining[idx + marker.len()..].find(m).map(|i| (m, i)))
                .min_by_key(|&(_, i)| i)
            {
                &remaining[idx + marker.len()..idx + marker.len() + next_marker.1]
            } else {
                &remaining[idx + marker.len()..]
            };

            // 处理多个定义
            let defs: Vec<&str> = def_part.split(',').collect();
            let prefix = match pos {
                PartOfSpeech::Noun => "[名]",
                PartOfSpeech::Verb => "[动]",
                PartOfSpeech::Vi => "[不及物动词]",
                PartOfSpeech::Vt => "[及物动词]",
                PartOfSpeech::Adjective => "[形]",
                _ => "[未分类]",
            };
            let definition = defs
                .iter()
                .map(|s| format!("{} {}", prefix, s.trim()))
                .collect::<Vec<_>>()
                .join("；");

            entries.push((pos, definition));
            remaining = &remaining[idx + marker.len() + def_part.len()..];
        } else {
            // 没有找到词性标记，整个作为定义
            entries.push((PartOfSpeech::Noun, format!("[未分类] {}", remaining)));
            break;
        }
    }

    Ok(entries)
}

#[cfg(test)]
mod parse_translation_tests {

    mod single_pos_tag {
        use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
        use crate::seeds::word::parse_translation;

        //"vt. 及物动词1；";
        #[test]
        fn parse_translation_single_pos_tag_with_noun_correct_parsing() {
            let translation = "n. 名词1；";
            let expected = vec![(PartOfSpeech::Noun, "[名] 名词1；".to_string())];
            assert_eq!(parse_translation(translation).unwrap(), expected);
        }

        //"vi. 及物动词1；";
        #[test]
        fn parse_translation_single_pos_tag_with_vi_correct_parsing() {
            let translation = "vi. 不及物动词1；";
            let expected = vec![(PartOfSpeech::Vi, "[不及物动词] 不及物动词1；".to_string())];
            assert_eq!(parse_translation(translation).unwrap(), expected);
        }

        //"vt. 及物动词1；"
        #[test]
        fn parse_translation_single_pos_tag_with_vt_correct_parsing() {
            let translation = "vt. 及物动词1；";
            let expected = vec![(PartOfSpeech::Vt, "[及物动词] 及物动词1；".to_string())];
            assert_eq!(parse_translation(translation).unwrap(), expected);
        }

        //"adj. 形容词1；"
        #[test]
        fn parse_translation_single_pos_tag_with_adjective_correct_parsing() {
            let translation = "adj. 形容词1；";
            let expected = vec![(PartOfSpeech::Adjective, "[形] 形容词1；".to_string())];
            assert_eq!(parse_translation(translation).unwrap(), expected);
        }

        //"未分类1；"
        #[test]
        fn parse_translation_single_pos_tag_no_pos_tag() {
            let translation = "未分类1；";
            let expected = vec![(PartOfSpeech::Noun, "[未分类] 未分类1；".to_string())];
            assert_eq!(parse_translation(translation).unwrap(), expected);
        }
    }

    mod multiple_pos_tags {
        use words_lingo::entity::sea_orm_active_enums::PartOfSpeech;
        use crate::seeds::word::parse_translation;

        //"n. 名词1；名词2；v. 动词1；vt. 及物动词1；vi. 不及物动词1；adj. 形容词1；";

    #[test]
    fn parse_translation_multiple_pos_tags_correct_parsing() {
        let translation =
            "n. 名词1；名词2；v. 动词1；vt. 及物动词1；vi. 不及物动词1；adj. 形容词1；";
        let expected = vec![
            (PartOfSpeech::Noun, "[名] 名词1；名词2；".to_string()),
            (PartOfSpeech::Verb, "[动] 动词1；".to_string()),
            (PartOfSpeech::Vt, "[及物动词] 及物动词1；".to_string()),
            (PartOfSpeech::Vi, "[不及物动词] 不及物动词1；".to_string()),
            (PartOfSpeech::Adjective, "[形] 形容词1；".to_string()),
        ];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }
    #[test]
    fn parse_translation_nested_pos_tags_correct_parsing() {
        let translation = "n. 名词1；v. 动词1；n. 名词2；";
        let expected = vec![
            (PartOfSpeech::Noun, "[名] 名词1；".to_string()),
            (PartOfSpeech::Verb, "[动] 动词1；".to_string()),
            (PartOfSpeech::Noun, "[名] 名词2；".to_string()),
        ];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }

    #[test]
    fn parse_translation_no_pos_tags_uncategorized() {
        let translation = "未分类的定义1；未分类的定义2；";
        let expected = vec![(
            PartOfSpeech::Noun,
            "[未分类] 未分类的定义1；未分类的定义2；".to_string(),
        )];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }

    #[test]
    fn parse_translation_empty_string_empty_result() {
        let translation = "";
        let expected: Vec<(PartOfSpeech, String)> = vec![];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }

    #[test]
    fn parse_translation_only_pos_tags_no_definitions() {
        let translation = "n. v. adj.";
        let expected = vec![
            (PartOfSpeech::Noun, "[名] ".to_string()),
            (PartOfSpeech::Verb, "[动] ".to_string()),
            (PartOfSpeech::Adjective, "[形] ".to_string()),
        ];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }

    #[test]
    fn parse_translation_only_definitions_no_pos_tags() {
        let translation = "定义1；定义2；";
        let expected = vec![(PartOfSpeech::Noun, "[未分类] 定义1；定义2；".to_string())];
        assert_eq!(parse_translation(translation).unwrap(), expected);
    }
    }

}