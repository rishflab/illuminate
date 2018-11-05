use std::net::UdpSocket;
use std::io::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use network::message::Message;
use bincode::serialize;

pub struct Client {
    socket: UdpSocket
}

impl Client {

    pub fn new(ip:Ipv4Addr, port: u16) -> Client {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        Client {
            socket: UdpSocket::bind(addr).expect("could not bind to socket"),
        }
    }

    pub fn send(&self, msg: Message, port: u16) -> Result<usize> {
        self.socket.send(&serialize(&msg).unwrap())
    }
}