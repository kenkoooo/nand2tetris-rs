extern crate nand2tetris_rs;

use std::env;

use nand2tetris_rs::{tools, vm};

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines = tools::read_file(&args[1]).unwrap();
    let lines = lines
        .trim()
        .split("\n")
        .map(|line| vm::parser::parse_one_line(line).unwrap())
        .collect();

    let assembly = vm::translator::translate(&lines, &args[1]).unwrap();
    for line in assembly.iter() {
        println!("{}", line);
    }
}
