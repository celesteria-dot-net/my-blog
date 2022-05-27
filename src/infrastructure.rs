use crate::domain::{model::*, repository::*};

use derive_new::new;

#[derive(new)]
pub struct PostRepositoryImpl {}

impl PostRepository for PostRepositoryImpl {
    fn find_by_id(&self, _id: PostId) -> anyhow::Result<Post> {
        Ok(Post::default())
    }

    fn list(&self) -> anyhow::Result<Vec<Post>> {
        Ok(vec![])
    }
}
