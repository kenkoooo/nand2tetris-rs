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

    let translator = vm::translator::Translator::new(&args[1], 256, 300, 400, 3000, 3010);
    let assembly = translator.translate(&lines).unwrap();
    for line in assembly.iter() {
        println!("{}", line);
    }
}
