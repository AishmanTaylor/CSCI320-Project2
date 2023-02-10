use std::{env::args, fs};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() != 2 {
        println!("Usage: duplicate inputname outputname");
    } else {
        fs::copy(arguments[0].as_str(), arguments[1].as_str());
    }
}