use super::{PostId, PostMeta};

use derive_new::new;

#[derive(new)]
pub struct Post {
    pub id: PostId,
    pub title: String,
    pub content: String,
    pub meta: PostMeta,
}
