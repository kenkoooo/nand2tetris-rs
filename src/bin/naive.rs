extern crate nand2tetris_rs;

use std::env;
use std::io::{stdin, stdout, Write};

use nand2tetris_rs::computer::cpu::{CPUTrait, CPU};
use nand2tetris_rs::computer::memory::MemoryTrait;
use nand2tetris_rs::emulator::emulator::Emulator;
use nand2tetris_rs::emulator::memory::EmulatedMemory;
use nand2tetris_rs::emulator::rom::ROM32K;
use nand2tetris_rs::{computer, emulator, tools};

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = tools::read_file(&args[1]).unwrap();
    let binary = tools::load_hack_binary(&content);

    let mut emulator = Emulator::<EmulatedMemory, CPU>::new(&binary);

    for i in 0.. {
        emulator.iterate();

        // render
        for r in 0..256 {
            for c in 0..512 {
                let screen_address = tools::convert_address(16384 + r * 32 + c / 16);
                let screen_word = emulator.memory.access([false; 16], false, screen_address);
                let is_black = screen_word[(c % 16) as usize];
                assert!(!is_black);
            }
        }
        println!("{}", i);
    }
}
