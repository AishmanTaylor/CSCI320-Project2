use std::{env::args, fs::File, io::Read};

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
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("(contents)");
    Ok(())
}