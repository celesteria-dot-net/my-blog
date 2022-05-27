#[derive(
    serde::Serialize, serde::Deserialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
pub struct Date {
    #[prost(int32, tag = "1")]
    pub year: i32,
    #[prost(int32, tag = "2")]
    pub month: i32,
    #[prost(int32, tag = "3")]
    pub day: i32,
}
#[derive(
    serde::Serialize, serde::Deserialize, derive_new::new, Clone, PartialEq, ::prost::Message,
)]
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {}
// message Post {
// ! This should be defined elsewere
// }

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostsRequest {
    /// The parent resource name, for example, "shelves/shelf1"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPostsResponse {
    /// The field name should match the noun "Post" in the method name.
    /// There will be a maximum number of items returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub posts: ::prost::alloc::vec::Vec<Post>,
    /// Token to retrieve the next page of results, or empty if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPostRequest {
    /// The field will contain name of the resource requested.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
