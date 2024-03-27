#[macro_use]
// an `extern crate` loading macros must be at the crate root
extern crate lazy_static;
mod rpc;
mod tun;
mod encrypt;
mod config;
mod connection;

use config::opt;
use connection::message;
use connection::conn;
use connection::server;
use prost::Message;
//from_args is a trait from StructOpt, if you want to use in different module, than you need to "use" it first -gs
// use structopt::StructOpt;

fn main() {
    println!("Hello, world!");

    let opt = opt::get_config();
    println!("{:#?}", opt);

    let mut msg = message::MessagePing{
        timestamp: 1234,
        local_addr: String::from("gadx"),
        local_private_addr: String::from("xxx"),
        ip: String::from("1.1.1.1"),
        dc: String::from("2.2.2.2"),
    };


    println!("{:#?}", msg.encode_to_vec())
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_connection() -> (){
        println!("connection")
    }
}