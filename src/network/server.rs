use std::net::*;
use std::io::Result;
use std::env;
use std::io::{Read, Write};

pub struct Server {
    listener: TcpListener,
    stream: TcpStream,
}

impl Server {
    pub fn new (stream_port: u16, listen_port: u16) -> Server {

        let stream_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), stream_port);
        let listen_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), listen_port);

        let listener = TcpListener::bind(listen_addr).unwrap();
        let stream = TcpStream::connect(stream_addr).unwrap();

        Server {
            listener: listener,
            stream: stream,
        }
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