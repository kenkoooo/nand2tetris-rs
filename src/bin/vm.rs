extern crate nand2tetris_rs;

use std::env;

use nand2tetris_rs::{tools, vm};

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines = tools::read_file(&args[1]).unwrap();
    let lines = lines.trim().split("\n").collect();
    let assembly = vm::compile(&lines).unwrap();
    for line in assembly.iter() {
        println!("{}", line);
    }
}
