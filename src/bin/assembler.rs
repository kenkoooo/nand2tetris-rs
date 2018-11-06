extern crate nand2tetris_rs;

use std::env;

use nand2tetris_rs::assembler::{formatter, model, parser};
use nand2tetris_rs::tools;

fn main() {
    let args: Vec<String> = env::args().collect();
    tools::read_file(&args[1])
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| parser::parse(line))
        .filter(|result| result != &Ok(model::Command::Comment))
        .map(|result| result.and_then(|cmd| formatter::format_to_binary(&cmd)))
        .collect::<Result<Vec<_>, ()>>()
        .unwrap()
        .iter()
        .for_each(|line| println!("{}", line));
}
