#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Date {
    #[prost(int32, tag = "1")]
    pub year: i32,
    #[prost(int32, tag = "2")]
    pub month: i32,
    #[prost(int32, tag = "3")]
    pub day: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct PostMeta {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub created_date: ::core::option::Option<Date>,
    #[prost(message, optional, tag = "6")]
    pub updated_date: ::core::option::Option<Date>,
}
