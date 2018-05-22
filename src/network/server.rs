use std::net::*;
use std::io::Result;
use std::env;
use std::io::{Read, Write};

struct Server {
    listener: TcpListener,
    sender: TcpStream,
}

impl Server {
    fn new (send_port: u16, listen_port: u16) -> Server {

    }
}