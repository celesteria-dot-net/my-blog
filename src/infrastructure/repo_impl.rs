use super::{model::PostMatter, shared::GeneratePost};
use crate::domain::{
    model::{Post, PostId},
    repository::*,
};

use derive_new::new;
use gray_matter::{engine::YAML, Matter};
use std::{fs, path::PathBuf};

#[derive(new, Clone, Default)]
pub struct PostRepositoryImpl {}

impl PostRepository for PostRepositoryImpl {
    fn find_by_id(&self, id: PostId) -> anyhow::Result<Post> {
        let path = PathBuf::from(format!("./static/{}.md", id.0));
        anyhow::ensure!(path.exists(), "The file is not found.");
        let str = fs::read_to_string(path)?;
        let matter = Matter::<YAML>::new();
        let parsed = matter
            .parse_with_struct::<PostMatter>(&str)
            .ok_or_else(|| anyhow::anyhow!("Failed to parse markdown."))?;

        Ok(Post::gen(id, parsed.data, parsed.content))
    }

    fn list(&self) -> anyhow::Result<Vec<Post>> {
        let list: Vec<Post> = fs::read_dir("./static")
            .unwrap()
            .map(|dir| dir.unwrap().path())
            .filter(|dir| dir.extension().and_then(|s| s.to_str()) == Some("md"))
            .map(|dir| {
                (
                    dir.file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                        .unwrap(),
                    fs::read_to_string(dir).unwrap(),
                )
            })
            .map(|(file_name, str)| {
                let matter = Matter::<YAML>::new();

                (
                    PostId::new(file_name),
                    matter.parse_with_struct::<PostMatter>(&str),
                )
            })
            .filter(|(id, matter)| id.is_ok() && matter.is_some())
            .map(|(id, matter)| (id.unwrap(), matter.unwrap()))
            .map(|(id, matter)| Post::gen(id, matter.data, matter.content))
            .collect();

        Ok(list)
    }
}
