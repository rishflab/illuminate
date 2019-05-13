//use std::net::*;
//use std::io::Result;
//use std::env;
//use std::io::{Read, Write};
//
//
//use blackhole::network::server::Server;
//
//
//fn parse_args(args: Vec<String>) -> Option<(u16, u16)> {
//
//     match args.len() {
//        3 => {
//            let arg1 = args[1].clone();
//            let arg2 = args[2].clone();
//            match (arg1.parse::<u16>(), arg2.parse::<u16>()) {
//                (Ok(a), Ok(b)) => Some((a, b)),
//                (_,_) => None,
//            }
//        },
//        _ => {
//            panic!("incorrect arguments")
//        },
//    }
//}

fn main() {

//    let args: Vec<String> = env::args().collect();
//
//    let ports = parse_args(args);
//
//    let (listen_port,send_port) = match ports {
//        Some((a,b)) => {
//            (a,b)
//        },
//        None => panic!(),
//    };
//
//    let server = Server::new(listen_port);

    //Server::send("hello", send_port);

    //Ok(())
}