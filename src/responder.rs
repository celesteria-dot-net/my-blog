use crate::domain::{Post, PostId, PostMeta};

use actix_web::{get, HttpResponse, Responder};
use gray_matter::{engine::YAML, Matter};
use std::fs;

#[get("/list")]
pub async fn list_posts() -> impl Responder {
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
                fs::read_to_string(dir),
            )
        })
        .map(|(file_name, str)| {
            (
                PostId::new(file_name),
                matter.parse_with_struct::<PostMeta>(&str.unwrap()).unwrap(),
            )
        })
        .filter(|(id, _)| id.is_ok())
        .map(|(id, mat)| Post::new(id.unwrap(), mat.content, mat.data))
        .collect();

    HttpResponse::Ok().json(list)
}
