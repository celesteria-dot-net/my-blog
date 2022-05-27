// /**
//  * protobufをビルドするためのファイル
//  */
fn main() {
    let json = include_str!("./build_config.json");
    prost_serde::build_with_serde(json);
    // prost_build::compile_protos(&["post_meta.proto", "post.proto"], &["protobuf/my-blog/v1"])
    //     .unwrap();
}
