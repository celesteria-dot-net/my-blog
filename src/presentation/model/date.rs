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
