use super::model::Post;
use crate::{infrastructure::PostRepositoryImpl, usecase::PostUseCase};

use actix_web::{get, web, HttpResponse, Responder};

fn post_use_case() -> PostUseCase<PostRepositoryImpl> {
    PostUseCase::new(PostRepositoryImpl::new())
}

#[get("/posts")]
pub async fn list_posts() -> impl Responder {
    match post_use_case().list() {
        Ok(list) => {
            let list = list
                .into_iter()
                .map(|post| post.into())
                .collect::<Vec<Post>>();
            HttpResponse::Ok().json(list)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/posts/{post_id}")]
pub async fn show_post(path: web::Path<String>) -> impl Responder {
    let post_id = path.into_inner();

    HttpResponse::Ok().body(format!("post_id is {}", post_id))
}
