use std::net::{TcpStream};
use std::sync::Arc;
use std::io;
use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct Conn{
    stream: Arc<TcpStream>,
    // stream: TcpStream,
}

impl Conn {
    pub fn new(s:TcpStream) -> Conn {
        Conn{
            stream: Arc::<TcpStream>::new(s),
            // stream: s,
        }
    }

    pub fn read_at_least(&self, size: usize) -> io::Result<Vec::<u8>>{
        // let mut buf = &mut [u8; size];
        // let mut buf = Vec::<u8>::with_capacity(size);
        let mut buf = vec![0 as u8; size];
        match self.stream.as_ref().read_exact(&mut buf){
            Ok(()) => Ok(buf),
            Err(e) => Err(e)
        }
        // let mut s = &self.stream;
        // match s.read_exact(&mut buf){
        //     Ok(()) => Ok(buf),
        //     Err(e) => Err(e)
        // }
    }

    pub fn write(&self, bytes :&Vec::<u8>) -> io::Result<usize> {
        match self.stream.as_ref().write(bytes){
            Ok(s) => Ok(s),
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_read_conn() -> (){
        if let Ok(stream) = TcpStream::connect("139.196.36.44:22") {
            println!("Connected to the server!");
            let c = Conn::new(stream);
            let result = c.read_at_least(10).unwrap();
            println!("{:#?}", result);
            let result = c.read_at_least(10).unwrap();
            println!("{:#?}", result);
        } else {
            println!("Couldn't connect to server...");
        }
        println!("conn")
    }

    #[test]
    fn test_write_conn() -> () {
        if let Ok(stream) = TcpStream::connect("127.0.0.1:9000") {
            println!("Connected to the server!");
            let c = Conn::new(stream);
            let chars : &Vec<char> = &vec!['a','b','c','d']; 
            let bytes = &chars.iter().map(|c| *c as u8).collect::<Vec<_>>();
            let result = c.write(bytes).unwrap();
            println!("{:#?}", result);
            let result = c.read_at_least(10).unwrap();
            println!("{:#?}", result);
        } else {
            println!("Couldn't connect to server...");
        }
    }
}