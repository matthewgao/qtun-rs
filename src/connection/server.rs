use std::net::{TcpStream, TcpListener};
use std::sync::{Arc};
use std::thread;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{Error, ErrorKind};
use super::conn::Conn;

#[derive(Debug)]
pub struct Server {
    addr: String,
    read_handler: fn (Arc<Conn>) -> io::Result<()>,
    write_handler: fn (Arc<Conn>) -> io::Result<()>
}

impl Server {
    pub fn new(addr : String, rf : fn (Arc<Conn>) -> io::Result<()>, wf : fn (Arc<Conn>) -> io::Result<()>) -> Server {
        Server{
            addr : addr,
            read_handler: rf,
            write_handler: wf,
        }
    }

    fn start_listening(&self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr.clone())?;
        // accept connections and process them serially
        for stream in listener.incoming() {
            self.process(stream?);
        }
        Ok(())
    }

    fn process(&self, stream: TcpStream) -> () {
        let conn = Arc::<Conn>::new(Conn::new(stream));
        let read_conn = conn.clone();
        let write_conn = conn.clone();
        let wh = self.write_handler;
        let rh = self.read_handler;
        thread::Builder::new().spawn(move || {
            loop {
                let conn = read_conn.clone();
                match (wh)(conn) {
                    Ok(()) => (),
                    Err(e) => {println!("{:#?}", e); return},
                };
            }
        });

        thread::Builder::new().spawn(move || {
            loop {
                let conn = write_conn.clone();
                match (wh)(conn) {
                    Ok(()) => (),
                    Err(e) => {println!("{:#?}", e); return},
                };
            }

        });
    }
}

mod tests{
    use super::*;
    #[test]
    fn test_start_server() -> (){
        let rf = |conn: Arc<Conn>| -> io::Result<()> {
            println!("Hello, world!");
            let result = conn.as_ref().read_at_least(10).unwrap();
            println!("{:#?}", result);
            // Ok(())
            Err(Error::new(ErrorKind::Other, "fail"))
        };

        let wf = |conn: Arc<Conn>| -> io::Result<()> {
            println!("Hello, world! write");
            let chars : &Vec<char> = &vec!['a','b','c','d']; 
            let bytes = &chars.iter().map(|c| *c as u8).collect::<Vec<_>>();
            let result = conn.as_ref().write(bytes).unwrap();
            println!("{:#?}", result);
            // Ok(())
            Err(Error::new(ErrorKind::Other, "fail 2"))
        };

        let server = Server::new("0.0.0.0:11111".to_string(),
         rf, wf
        );
        server.start_listening();
        println!("server exit")
    }
}