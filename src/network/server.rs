use std::net::*;
use std::io::Result;
use std::io::{Read, Write};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u16) -> Server {


        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

        let listener = TcpListener::bind(addr).unwrap();

        Server {
            listener: listener,
        }
    }

    pub fn send(msg: &str, port: u16) -> Result<usize> {

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

        let mut stream = TcpStream::connect(addr).unwrap();

        stream.write(msg.as_bytes())

    }

    fn handle_client(mut stream: TcpStream) {
        let buf = String::new();
        let incoming = stream.read_to_string(&mut buf.clone()).unwrap();
    //    match incoming {
    //        Ok(t) => {
    //            println!("{}", t);
    //        },
    //        Err(e) => {
    //            println!("{}", e);
    //        }
    //    }
        println!("{:?}", incoming)
    }


}