use std::net::{TcpListener, TcpStream};
use std::io::Result;
use std::env;
use std::io::{Read, Write};


fn handle_client(mut stream: TcpStream) {
    let buf = String::new();
    let incoming = stream.read_to_string(&mut buf.clone());
    match incoming {
        Ok(t) => {
            println!("{}", t);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}



enum Purpose {
    Listener,
    Sender,
}

impl Purpose {
    fn parse (string: String) -> Option<Purpose> {
        match &*string {
            "listen" => Some(Purpose::Listener),
            "sender" => Some(Purpose::Sender),
            _ => None,
        }
    }
}

fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut send_port = "80".to_string();

    let mut port = "3000".to_string();

    let mut purpose =  Purpose::Listener;

    let mut localhost = "127.0.0.1".to_owned();

    match args.len() {
        3 => {
            if let Some(arg) = Purpose::parse(args[1].clone()) {
                purpose = arg;
                port = args[2].clone();
            } else {
                eprintln!("first argument is invalid");
                return Ok(());
            };
        },
        _ => {
            eprintln!("incorrect number of arguments");
            return Ok(());
        },
    }

    match purpose {
        Purpose::Listener => {
            let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

            loop {
            // accept connections and process them serially
                for stream in listener.incoming() {
                    match stream {
                        Ok(b) => {
                            handle_client(b);
                        },
                        Err(e) => {
                            println!("TCP stream error!")
                        }
                    }
                    //handle_client(&stream?);
                }
            }
        },
        Purpose::Sender => {
            let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

            let _ = stream.write(b"hello");

        },
    }

    Ok(())
}