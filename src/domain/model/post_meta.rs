use chrono::NaiveDate;
use derive_new::new;

#[derive(new)]
pub struct PostMeta {
    pub description: Option<String>,
    pub thumbnail: Option<String>,
    pub tags: Vec<String>,
    pub created_date: NaiveDate,
    pub updated_date: Option<NaiveDate>,
}
