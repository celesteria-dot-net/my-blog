use super::model::*;
use crate::domain::model::*;

use chrono::NaiveDate;
use regex::Regex;

pub const FORMAT: &str = "%Y-%m-%d";

pub trait GeneratePost {
    fn gen(id: PostId, meta: PostMatter, content: String) -> Post;
}

impl GeneratePost for Post {
    fn gen(id: PostId, matter: PostMatter, content: String) -> Post {
        let regex = Regex::new(r"^(\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01]))$").unwrap();
        let created_date = regex.find(&id.0).unwrap().as_str();
        let meta = PostMeta::new(
            matter.description,
            matter.thumbnail,
            matter.tags.unwrap_or_default(),
            NaiveDate::parse_from_str(created_date, FORMAT).unwrap(),
            matter.updated_date,
        );

        Post::new(id, matter.title, content, meta)
    }
}
