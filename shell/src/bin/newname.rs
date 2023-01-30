use std::fs;

fn main() {
    // Sourced from: https://www.tutorialspoint.com/rust/rust_input_output.htm
    let mut inputName = String::new();
    println!("Enter the name of the file you want to change: ");
    let mut outputName = String::new();
    println!("Enter the new name of the file: ");
    // Sourced from: https://doc.rust-lang.org/std/fs/fn.rename.html 
    fs::rename(inputName, outputName);
}