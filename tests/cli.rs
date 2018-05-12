extern crate teleport;

use std::process::Command;

#[test]
fn prints_stdin() {
     let output = Command::new("./target/debug/teleport")
            .output().expect("");

        assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, world!\n");
}