extern crate teleport;

use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::error::Error;

//#[test]
//fn prints_stdin() {
//     let output = Command::new("./target/debug/teleport")
//            .output().expect("");
//
//        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
//}


#[test]
fn say_hello() {
    let mut listener = Command::new("./target/debug/cli_parsing")
        .arg("3000")
        .arg("3001")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

//    let mut sender = Command::new("./target/debug/teleport")
//        .arg("3001")
//        .arg("3000")
//        .spawn()
//        .expect("failed to execute child");

//    let mut s = String::new();
//    match listener.stdout.unwrap().read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read wc stdout: {}",
//                           why.description()),
//         Ok(_) => print!("wc responded with:\n{}", s),
//
//    }
//
//    assert_eq!(s, "Goodbye")

}
