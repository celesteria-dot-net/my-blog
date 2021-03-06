#[derive(
    serde::Deserialize, serde::Serialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct PostMeta {
    #[prost(string, optional, tag = "1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: ::core::option::Option<super::date::Date>,
    #[prost(message, optional, tag = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: ::core::option::Option<super::date::Date>,
}
