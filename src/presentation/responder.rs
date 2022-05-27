use crate::{
    domain::model::{Post, PostId, PostMeta},
    infrastructure::PostRepositoryImpl,
    usecase::PostUseCase,
};

use actix_web::{get, web, HttpResponse, Responder};
use gray_matter::{engine::YAML, Matter};
use std::fs;

fn post_use_case() -> PostUseCase<PostRepositoryImpl> {
    PostUseCase::new(PostRepositoryImpl::new())
}

#[get("/posts")]
pub async fn list_posts() -> impl Responder {
    // let matter = Matter::<YAML>::new();
    // let list: Vec<Post> = fs::read_dir("./static")
    //     .unwrap()
    //     .map(|dir| dir.unwrap().path())
    //     .filter(|dir| dir.extension().and_then(|s| s.to_str()) == Some("md"))
    //     .map(|dir| {
    //         (
    //             dir.file_stem()
    //                 .and_then(|s| s.to_str())
    //                 .map(|s| s.to_string())
    //                 .unwrap(),
    //             fs::read_to_string(dir),
    //         )
    //     })
    //     .map(|(file_name, str)| {
    //         (
    //             PostId::new(file_name),
    //             matter.parse_with_struct::<PostMeta>(&str.unwrap()).unwrap(),
    //         )
    //     })
    //     .filter(|(id, _)| id.is_ok())
    //     .map(|(id, mat)| Post::new(id.unwrap(), mat.content, mat.data))
    //     .collect();

    HttpResponse::Ok()
}

#[get("/posts/{post_id}")]
pub async fn show_post(path: web::Path<String>) -> impl Responder {
    let post_id = path.into_inner();

    HttpResponse::Ok().body(format!("post_id is {}", post_id))
}
