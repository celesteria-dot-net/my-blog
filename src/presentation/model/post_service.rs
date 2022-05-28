#[derive(
    serde::Deserialize, serde::Serialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct ListPostsRequest {}
#[derive(
    serde::Deserialize, serde::Serialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct ListPostsResponse {
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<super::post::Post>,
}
#[derive(
    serde::Deserialize, serde::Serialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct GetPostRequest {
    #[prost(string, tag = "1")]
    pub post_id: ::prost::alloc::string::String,
}
