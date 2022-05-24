use super::{PostId, PostMeta};

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize)]
pub struct Post {
    id: PostId,
    content: String,
    meta: PostMeta,
}
