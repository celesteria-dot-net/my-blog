use super::{
    model::{GetPostRequest, Post},
    server::Context,
};
use crate::{domain::model::PostId, usecase::PostUseCase};

use actix_web::{get, web, HttpResponse, Responder};

// TODO: エラーをまとめる

#[get("/posts")]
pub async fn list_posts(data: web::Data<Context>) -> impl Responder {
    match PostUseCase::new(data.repo.clone()).list() {
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
pub async fn show_post(
    path: web::Path<GetPostRequest>,
    data: web::Data<Context>,
) -> impl Responder {
    let path = path.into_inner();
    let post_id = match PostId::new(path.post_id) {
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match PostUseCase::new(data.repo.clone()).find_by_id(post_id) {
        Ok(post) => {
            let post: Post = post.into();
            HttpResponse::Ok().json(post)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
