use std::env;

mod scanner;
use crate::scanner::*;

fn run(filename: String) {
    let mut scanner = scanner::Scanner { 
        error: false,
        source: "".to_string()
    };
    scanner.read(filename);
    scanner.scan();

    if scanner.error {
        println!("error occurred")
    }
}

fn main() {
    println!("Crafted Interpreter");
    let filename = env::args().nth(1).expect("no file given");
    run(filename);
}
