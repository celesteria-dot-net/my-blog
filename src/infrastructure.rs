use crate::domain::{model::*, repository::*};

use derive_new::new;
use gray_matter::{engine::YAML, Matter};
use std::fs;

#[derive(new)]
pub struct PostRepositoryImpl {}

impl PostRepository for PostRepositoryImpl {
    fn find_by_id(&self, _id: PostId) -> anyhow::Result<Post> {
        Ok(Post::default())
    }

    fn list(&self) -> anyhow::Result<Vec<Post>> {
        let matter = Matter::<YAML>::new();
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
            .inspect(|(id, str)| println!("{:?} {:?}", id, str))
            .map(|(file_name, str)| {
                (
                    PostId::new(file_name),
                    matter.parse_with_struct::<PostMeta>(&str),
                )
            })
            .inspect(|(id, meta)| println!("{} {}", id.is_ok(), meta.is_some()))
            .filter(|(id, meta)| id.is_ok() && meta.is_some())
            .map(|(id, meta)| (id.unwrap(), meta.unwrap()))
            .map(|(id, meta)| Post::new(id, meta.content, meta.data))
            .collect();

        Ok(list)
    }
}
