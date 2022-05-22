use crate::domain::{Post, PostMeta};

use actix_web::{HttpResponse, Responder};
use gray_matter::{engine::YAML, Matter};
use std::fs;

pub async fn list_posts() -> impl Responder {
    let matter = Matter::<YAML>::new();
    let list: Vec<Post> = fs::read_dir("./static")
        .unwrap()
        .map(|dir| dir.unwrap().path())
        .map(fs::read_to_string)
        .map(|str| matter.parse_with_struct::<PostMeta>(&str.unwrap()).unwrap())
        .map(|mat| Post {
            content: mat.content,
            meta: mat.data,
        })
        .collect();

    HttpResponse::Ok().json(list)
}
