use std::{env::args, fs::{self}};

fn main() -> std::io::Result<()>{
    for arg in args().skip(1) {
        fs::remove_file(arg)?;
    }
    Ok(())
}