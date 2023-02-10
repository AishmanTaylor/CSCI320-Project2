use std::{env::args, fs::{File}, io::{BufReader, BufRead}};

fn main() -> anyhow::Result<()>{
    let mut numLines = 10;
    for arg in args().skip(1) {
        if arg.starts_with("-"){
            numLines = arg[1..].parse::<usize>()?;
        } else {
            let file = File::open(arg)?;
            let buffer = BufReader::new(file);
            for (i, line) in buffer.lines().enumerate() {
                if i < numLines {
                    println!("{}", line?);
                }
            }
        }
    }
    Ok(())
}