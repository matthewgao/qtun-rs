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
//from_args is a trait from StructOpt, if you want to use in different module, than you need to "use" it first -gs
// use structopt::StructOpt;

fn main() {
    println!("Hello, world!");

    let opt = opt::get_config();
    println!("{:#?}", opt);

    let mut msg = message::MessagePing::new();
    msg.Timestamp = 10000;

    println!("{:#?}", msg)
}
