use std::net::{TcpStream};
use std::sync::Arc;
use std::io;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct Client {
    addr: String,
    read_handler: fn (Arc<Conn>) -> io::Result<()>,
    write_handler: fn (Arc<Conn>) -> io::Result<()>
}

impl Client {
    pub fn new() {
        
    }

    fn connect(&self) -> io::Result<()> {

    }

    fn process(&self) -> () {
        
    }
}