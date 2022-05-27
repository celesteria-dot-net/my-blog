use crate::domain::model::{Post, PostId};

pub trait PostRepository {
    fn find_by_id(&self, id: PostId) -> anyhow::Result<Post>;
    fn list(&self) -> anyhow::Result<Vec<Post>>;
}
