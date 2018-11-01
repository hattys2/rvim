extern crate getch;

use getch::Getch;
use std::env;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;


fn main() {
    let output = Command::new("clear")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    println!("{}", String::from_utf8_lossy(&output.stdout));

    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       why.description()),
            Ok(_) => print!("{}", s),
        }
    }


    let g = Getch::new();

    match g.getch().unwrap() {
        _ => println!("none"),
    }

}
