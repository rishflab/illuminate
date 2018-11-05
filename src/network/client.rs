use std::net::UdpSocket;
use std::io::Result;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use super::rpc::Rpc;

pub struct Client;

impl Client {

    pub fn send<T: Rpc>(msg: T, port: u16) -> Result<usize> {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        let socket = UdpSocket::bind(addr)?;
        socket.send(msg.to_bytes())
    }
}