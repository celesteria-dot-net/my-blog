use chrono::NaiveDate;
use derive_new::new;
use serde::Deserialize;

#[derive(new, Deserialize, PartialEq, Debug, Clone)]
pub struct PostMeta {
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(deserialize_with = "serde_naive_date::deserialize")]
    pub created_date: NaiveDate,
    #[serde(default)]
    #[serde(deserialize_with = "serde_option_naive_date::deserialize")]
    pub updated_date: Option<NaiveDate>,
}

impl Default for PostMeta {
    fn default() -> Self {
        Self::new(
            String::default(),
            None,
            None,
            None,
            NaiveDate::from_ymd(1900, 1, 1),
            None,
        )
    }
}

const FORMAT: &str = "%Y-%m-%d";

mod serde_naive_date {
    use super::FORMAT;

    use chrono::NaiveDate;
    use serde::{self, de, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        NaiveDate::parse_from_str(&s, FORMAT)
            .map_err(|e| de::Error::custom(format!("chrono returned an error: {}", e)))
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

    fn deserialize(str: &str) -> Option<ParsedEntityStruct<PostMeta>> {
        deserialize_with::<PostMeta>(str)
    }

    #[test]
    fn only_required_fields() {
        let expected = PostMeta {
            title: "title".to_string(),
            created_date: NaiveDate::from_ymd(2020, 1, 1),
            ..PostMeta::default()
        };
        let str = formatdoc! {"
            ---
            title: {title}
            created_date: {created_date}
            ---
            content!
            ",
            title = expected.title,
            created_date = expected.created_date.format(FORMAT)
        };
        let found = deserialize(&str);

        assert_eq!(found.map(|d| d.data), Some(expected));
    }

    #[test]
    fn all_fields() {
        let expected = PostMeta::new(
            "title".to_string(),
            Some("description".to_string()),
            Some("image".to_string()),
            Some(
                vec!["image1", "image2"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            ),
            NaiveDate::from_ymd(2020, 1, 1),
            Some(NaiveDate::from_ymd(2020, 1, 1)),
        );
        let str = formatdoc! { "
                ---
                title: {title}
                description: {description}
                image: {image}
                tags: {tags:?}
                created_date: {created_date}
                updated_date: {updated_date}
                ---
                content!
            ",
            title = expected.title,
            description = expected.clone().description.unwrap(),
            image = expected.clone().image.unwrap(),
            tags = expected.clone().tags.unwrap(),
            created_date = expected.created_date.format(FORMAT),
            updated_date = expected.updated_date.unwrap().format(FORMAT)
        };
        let found = deserialize(&str);

        assert_eq!(found.map(|d| d.data), Some(expected));
    }
}
