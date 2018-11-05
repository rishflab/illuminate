#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate teleport;

use std::net::UdpSocket;
use bincode::{deserialize, serialize};
use teleport::network::messages::Point2;

fn main() -> std::io::Result<()> {

    let mut socket = UdpSocket::bind("127.0.0.1:34251")?;

    let p =  Point2{x: 32.30, y:12.12};
    let encoded: &[u8] = &serialize(&p).unwrap();
    assert_eq!(encoded.len(), 4 + 4);

    let decoded: Point2 = deserialize(&encoded[..]).unwrap();

    assert_eq!(p, decoded);

    println!("{:?}", decoded);


    socket.send_to(encoded, "127.0.0.1:34254")?;


    Ok(())
}
//fn main() -> std::io::Result<()> {
//    {
//        let mut socket = UdpSocket::bind("127.0.0.1:34254")?;
//
//        // Receives a single datagram message on the socket. If `buf` is too small to hold
//        // the message, it will be cut off.
//        let mut buf = [0; 10];
//        let (amt, src) = socket.recv_from(&mut buf)?;
//
//        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
//        let buf = &mut buf[..amt];
//        buf.reverse();
//        socket.send_to(buf, &src)?;
//    } // the socket is closed here
//    Ok(())
//}