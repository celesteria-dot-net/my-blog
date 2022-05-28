mod post;
pub use post::Post;

mod post_meta;
pub use post_meta::PostMeta;

mod date;
use date::Date;

mod converter {
    use super::*;

    use chrono::{Datelike, NaiveDate};

    impl From<crate::domain::model::Post> for Post {
        fn from(post: crate::domain::model::Post) -> Self {
            Self::new(post.id.0, post.title, post.content, Some(post.meta.into()))
        }
    }

    impl From<crate::domain::model::PostMeta> for PostMeta {
        fn from(meta: crate::domain::model::PostMeta) -> Self {
            Self::new(
                meta.description,
                meta.thumbnail,
                meta.tags,
                Some(meta.created_date.into()),
                meta.updated_date.map(|date| date.into()),
            )
        }
    }

    impl From<NaiveDate> for Date {
        fn from(date: NaiveDate) -> Self {
            Self::new(
                date.year(),
                date.month().try_into().unwrap(),
                date.day().try_into().unwrap(),
            )
        }
    }
}
