use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    for words in entries.iter(){
        println!("{}", words.to_str().unwrap());
    }
    Ok(())
}