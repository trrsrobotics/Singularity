// Copyright © The Roman Ridge School 2023-Present

use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut file_path = String::new();
    let stdin = io::stdin;
    println!("Enter file path to instructions:");
    stdin.read_line(&mut file_path)?;
    println!("Reading instructions from file: {} ", file_path);
    let instr = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("Read instructions from file correctly!");
    // todo => write assembler

    Ok(())
}
