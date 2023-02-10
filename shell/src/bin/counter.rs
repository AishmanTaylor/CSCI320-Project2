use std::{env::args, fs::{File, self}, io::{Read, BufReader, BufRead}, process::Command};

fn main() -> std::io::Result<()>{
    for arg in args().skip(1) {
        match dump(arg.as_str()) {
            Ok(_) => {}
            Err(e) => {
                print!("Error when processing file {arg}: {e}");
            }
        }
    }
    Ok(())
}

fn dump(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let buffer = BufReader::new(file);
    let mut lineCount = 0;
    let mut wordCount = 0;
    let mut charCount = 0;
    for line in buffer.lines() { 
        lineCount += 1;
        let line = line?;
        charCount += line.len() + 1;
        for word in line.split_whitespace() {
            wordCount += 1;
        }
    }
    println!("Line Count: {}, Word Count: {}, Character Count: {}", lineCount, wordCount, charCount);
    Ok(())
}