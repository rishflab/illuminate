use std::net::UdpSocket;
use cgmath::{Point2, Vector2};

fn parse_args(args: Vec<String>) -> (u16, String) {

     match args.len() {
        3 => {
            let arg1 = args[1].clone();
            let arg2 = args[2].clone();
            match (arg1.parse::<u16>(), arg2.parse::<u16>()) {
                (Ok(a), Ok(b)) => Some((a, b)),
                (_,_) => None,
            }
        },
        _ => {
            panic!("incorrect number of arguments")
        },
    }
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let ports = parse_args(args);

    let mut socket = UdpSocket::bind("127.0.0.1:34254")?;

    Server::send("hello", send_port);

    socket.send_to(buf, &src)?;

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