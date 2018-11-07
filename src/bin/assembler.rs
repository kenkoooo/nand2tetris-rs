extern crate nand2tetris_rs;

use std::env;

use nand2tetris_rs::assembler;

fn main() {
    let args: Vec<String> = env::args().collect();

    let lines = assembler::assemble(&args[1]).unwrap();
    for line in lines.iter() {
        println!("{}", line);
    }
}
