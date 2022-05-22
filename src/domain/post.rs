use super::PostMeta;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Post {
    pub content: String,
    pub meta: PostMeta,
}
