// Protobufの内容を読み込む
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

pub mod domain;
pub mod responder;
pub mod utils;
