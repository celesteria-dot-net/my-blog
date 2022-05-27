use super::{PostId, PostMeta};

use derive_new::new;

#[derive(new, Default)]
pub struct Post {
    pub id: PostId,
    pub content: String,
    pub meta: PostMeta,
}
