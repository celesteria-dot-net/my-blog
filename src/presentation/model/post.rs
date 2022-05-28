#[derive(
    serde::Serialize, serde::Deserialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct Post {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub thumbnail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub created_date: ::core::option::Option<super::date::Date>,
    #[prost(message, optional, tag = "8")]
    pub updated_date: ::core::option::Option<super::date::Date>,
}
