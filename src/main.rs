use std::net::*;
use std::io::Result;
use std::env;
use std::io::{Read, Write};


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



enum Purpose {
    Listener,
    Sender,
}

impl Purpose {
    fn parse (string: String) -> Purpose {
        match &*string {
            listener => Purpose::Listener,
            sender => Purpose::Sender,
        }
    }
}


fn parse_args(args: Vec<String>) -> Some((Purpose, u16)) {

     match args.len() {
        3 => {
            purpose = Purpose::parse(args[2].clone());
            port = args[1].clone();
        },
        _ => {
            None
        },
    }
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut send_port = "80".to_string();

    let mut port = "3000".to_string();

    let mut purpose =  Purpose::Sender;

    let mut localhost = "127.0.0.1".to_owned();


    eprintln!("{}", port);

    let port_int = port.parse::<u16>().unwrap();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port_int);


    match purpose {
        Purpose::Listener => {

            let listener = TcpListener::bind(addr).unwrap();


            for stream in listener.incoming() {
                match stream {
                    Ok(b) => {
                        //println!("ewfwefwe");
                        handle_client(b);
                    },
                    Err(e) => {
                        println!("TCP stream error!")
                    }
                }
            }


        },
        Purpose::Sender => {

            println!("sending");

            let mut stream = TcpStream::connect(addr).unwrap();


            let _ = stream.write(b"hello");

        },
    }

    Ok(())
}