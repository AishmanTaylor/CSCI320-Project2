use std::{env::args, fs::{File, self}, io::Read, process::Command};

fn main() -> std::io::Result<()>{
    for arg in args().skip(1) {
        match dump(arg.as_str()) {
            Ok(_) => {fs::remove_file(file!())?}
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
    Command::new("cmd")
            .args(["wc -lw {file}"])
            .output()
            .expect("failed to execute process");
    Ok(())
}