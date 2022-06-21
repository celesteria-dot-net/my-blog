use crate::domain::{
    model::{Post, PostId},
    repository::{PostRepository, RepositoriesModuleExt},
};

use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct PostUseCase<R: RepositoriesModuleExt>(Arc<R>);

impl<R: RepositoriesModuleExt> PostUseCase<R> {
    pub fn find_by_id(&self, id: PostId) -> anyhow::Result<Post> {
        self.0.post_repo().find_by_id(id)
    }

    pub fn list(&self) -> anyhow::Result<Vec<Post>> {
        self.0.post_repo().list()
    }
}
