use std::env;
use std::process::Command;

fn main() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("{}", String::from_utf8_lossy(&output.stdout));

    for arg in env::args() {
        println!("{}", arg);
    }
}
