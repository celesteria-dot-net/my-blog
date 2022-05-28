use super::shared::FORMAT;

use chrono::NaiveDate;
use derive_new::new;
use serde::Deserialize;

#[derive(new, Deserialize, PartialEq, Debug, Clone)]
pub struct PostMatter {
    pub title: String,
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "serde_option_naive_date::deserialize")]
    pub updated_date: Option<NaiveDate>,
}

impl Default for PostMatter {
    fn default() -> Self {
        Self::new(String::default(), None, None, None, None)
    }
}

mod serde_option_naive_date {
    use super::FORMAT;

    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;

        Ok(match s {
            Some(s) => {
                Some(NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?)
            }
            None => None,
        })
    }
}

#[cfg(test)]
mod deserialize_test {
    use super::*;

    use chrono::NaiveDate;
    use gray_matter::{engine::YAML, Matter, ParsedEntityStruct};
    use indoc::formatdoc;
    use serde::de::DeserializeOwned;

    fn deserialize_with<T: DeserializeOwned>(str: &str) -> Option<ParsedEntityStruct<T>> {
        let matter = Matter::<YAML>::new();

        matter.parse_with_struct::<T>(str)
    }

    fn deserialize(str: &str) -> Option<ParsedEntityStruct<PostMatter>> {
        deserialize_with::<PostMatter>(str)
    }

    #[test]
    fn only_required_fields() {
        let expected = PostMatter {
            title: "title".to_string(),
            ..PostMatter::default()
        };
        let str = formatdoc! {"
            ---
            title: {title}
            ---
            content!
            ",
            title = expected.title,
        };
        let found = deserialize(&str);

        assert_eq!(found.map(|d| d.data), Some(expected));
    }

    #[test]
    fn all_fields() {
        let expected = PostMatter::new(
            "title".to_string(),
            Some("description".to_string()),
            Some("thumbnail".to_string()),
            Some(
                vec!["thumbnail1", "thumbnail2"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
            Some(NaiveDate::from_ymd(2020, 1, 1)),
        );
        let str = formatdoc! { "
                ---
                title: {title}
                description: {description}
                thumbnail: {thumbnail}
                tags: {tags:?}
                updated_date: {updated_date}
                ---
                content!
            ",
            title = expected.title,
            description = expected.clone().description.unwrap(),
            thumbnail = expected.clone().thumbnail.unwrap(),
            tags = expected.clone().tags.unwrap(),
            updated_date = expected.updated_date.unwrap().format(FORMAT)
        };
        let found = deserialize(&str);

        assert_eq!(found.map(|d| d.data), Some(expected));
    }
}
