#[derive(
    serde::Serialize, serde::Deserialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub content: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub meta: ::core::option::Option<super::post_meta::PostMeta>,
}
