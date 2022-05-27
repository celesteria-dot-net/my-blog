use super::{PostId, PostMeta};

use derive_new::new;
use serde::Deserialize;

#[derive(new, Default, Deserialize)]
pub struct Post {
    pub id: PostId,
    pub content: String,
    pub meta: PostMeta,
}
