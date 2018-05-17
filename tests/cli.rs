extern crate teleport;

use std::process::Command;

//#[test]
//fn prints_stdin() {
//     let output = Command::new("./target/debug/teleport")
//            .output().expect("");
//
//        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
//}
//
//#[test]
//fn chat() {
//
//    let output1 = Command::new("./target/debug/teleport")
//        .args("Hello".chars())
//        .output().expect("");
//
//
//    let output2 = Command::new("./target/debug/teleport")
//        .args("Goodbye".chars())
//        .output().expect("");
//
//    assert_eq!(String::from_utf8_lossy(&output1.stdout), "Goodbye")
//
//}