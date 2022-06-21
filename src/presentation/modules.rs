use crate::{
    domain::repository::RepositoriesModuleExt, infrastructure::modules::RepositoriesModule,
    usecase::PostUseCase,
};

use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct Modules {
    post_use_case: PostUseCase<RepositoriesModule>,
}

impl Default for Modules {
    fn default() -> Self {
        Self::new(PostUseCase::new(Arc::new(RepositoriesModule::new())))
    }
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn post_use_case(&self) -> &PostUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn post_use_case(&self) -> &PostUseCase<Self::RepositoriesModule> {
        &self.post_use_case
    }
}
