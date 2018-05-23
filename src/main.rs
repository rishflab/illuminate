use std::net::*;
use std::io::Result;
use std::env;
use std::io::{Read, Write};

extern crate teleport;

use teleport::network::server::Server;
use teleport::world::state::*;



fn parse_args(args: Vec<String>) -> Option<(u16, u16)> {

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
            None
        },
    }
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let server = Server::new(3000, 3001);

    Ok(())
}