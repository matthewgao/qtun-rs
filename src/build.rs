extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    println!("start to build");
    protoc_rust::Codegen::new()
        .out_dir("src/connection")
        .inputs(&["connection/proto.proto"])
        .include("protos")
        .run()
        .expect("protoc");
}