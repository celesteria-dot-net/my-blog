use super::PostRepositoryImpl;
use crate::domain::repository::RepositoriesModuleExt;

use derive_new::new;

#[derive(new)]
pub struct RepositoriesModule {
    #[new(default)]
    post_repo: PostRepositoryImpl,
}

impl RepositoriesModuleExt for RepositoriesModule {
    type PostRepo = PostRepositoryImpl;

    fn post_repo(&self) -> &Self::PostRepo {
        &self.post_repo
    }
}
