// extern crate protoc_rust;

// use protoc_rust::Customize;

// fn main() {
//     println!("start to build");
//     protoc_rust::Codegen::new()
//         .out_dir("src/connection")
//         .inputs(&["src/connection/message.proto"])
//         .include("src/connection")
//         .run()
//         .expect("protoc");
// }

use prost_build::Config;
use std::io::Result;
fn main() -> Result<()> {
    let mut prost_config = Config::new();
    prost_config.out_dir("src/connection");
    prost_config.compile_protos(&["src/connection/message.proto"], &["src/"])?;
    Ok(())
}