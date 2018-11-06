extern crate nand2tetris_rs;

use std::env;

use nand2tetris_rs::assembler::model::Command;
use nand2tetris_rs::assembler::{formatter, parser};
use nand2tetris_rs::tools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source_code = tools::read_file(&args[1]).unwrap();

    let commands = parser::parse(&source_code).unwrap();
    let binary = formatter::format_to_binary(&commands).unwrap();
    for binary_line in binary.iter() {
        println!("{}", binary_line);
    }
}
