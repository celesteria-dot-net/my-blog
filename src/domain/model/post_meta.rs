use chrono::NaiveDate;
use derive_new::new;
use serde::Deserialize;

#[derive(new, Deserialize)]
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
