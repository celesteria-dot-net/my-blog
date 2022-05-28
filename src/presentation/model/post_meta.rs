#[derive(serde::Serialize, derive_new::new, Clone, PartialEq, ::prost::Message)]
pub struct PostMeta {
    #[prost(string, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub thumbnail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub created_date: ::core::option::Option<super::date::Date>,
    #[prost(message, optional, tag = "5")]
    pub updated_date: ::core::option::Option<super::date::Date>,
}
