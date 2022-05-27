use crate::domain::{
    model::{Post, PostId},
    repository::PostRepository,
};

use derive_new::new;

#[derive(new)]
pub struct PostUseCase<R: PostRepository>(R);

impl<R: PostRepository> PostUseCase<R> {
    pub fn find_by_id(&self, id: PostId) -> anyhow::Result<Post> {
        self.0.find_by_id(id)
    }

    pub fn list(&self) -> anyhow::Result<Vec<Post>> {
        self.0.list()
    }
}
