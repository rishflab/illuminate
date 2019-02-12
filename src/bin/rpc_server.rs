#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate blackhole;

use std::{thread, time};

use std::net::UdpSocket;
use bincode::{deserialize, serialize};
use blackhole::network::messages::Point2;


fn main() -> std::io::Result<()> {

    let mut socket = UdpSocket::bind("127.0.0.1:34254")?;
    let mut buf = [0; 1024];


    loop {
        let mut finished = false;

        //while finished == false {
        for _ in 1..5 {
            match socket.recv_from(&mut buf) {
                Ok((amt, src)) => {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    let decoded: Point2 = deserialize(&buf[..]).unwrap();
                    println!("{:?}", decoded );
                   
                },
                Err(e) =>{
                    println!("couldn't recieve a datagram: {}", e);
                    finished = true;
                }
            }
        }
        let delay = time::Duration::from_millis(5000);

        thread::sleep(delay);
    }
       


    Ok(())
}