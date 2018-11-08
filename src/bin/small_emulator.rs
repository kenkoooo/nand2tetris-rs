extern crate nand2tetris_rs;

use std::env;
use std::io::{stdin, stdout, Write};

use nand2tetris_rs::computer::cpu::{CPUTrait, CPU};
use nand2tetris_rs::computer::memory::MemoryTrait;
use nand2tetris_rs::emulator::memory::EmulatedMemory;
use nand2tetris_rs::emulator::rom::ROM32K;
use nand2tetris_rs::{computer, emulator, tools};

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = tools::read_file(&args[1]).unwrap();
    let binary = tools::load_hack_binary(&content);

    let mut rom = ROM32K::<EmulatedMemory>::new(&binary);
    let mut cpu = CPU::new();
    let mut memory = EmulatedMemory::new();

    memory.access(convert_address16(1), true, convert_address(0));
    memory.access(convert_address16(2), true, convert_address(0));

    let mut instruction = [false; 16];
    let mut in_m = [false; 16];
    let mut reset = true;
    for _ in 0..100 {
        cpu.tick(in_m, instruction, reset);
        let (out_m, write_m, address_m, pc) = cpu.tock(in_m, instruction);
        in_m = memory.access(out_m, write_m, address_m);
        instruction = rom.access(pc);

        println!(
            "{:?}",
            memory.access([false; 16], false, convert_address(2))
        );

        reset = false;
    }
}

fn convert_address(address: u16) -> [bool; 15] {
    let mut result = [false; 15];
    for i in 0..15 {
        result[i] = (1 << i) & address != 0;
    }
    result
}

fn convert_address16(address: u16) -> [bool; 16] {
    let mut result = [false; 16];
    for i in 0..16 {
        result[i] = (1 << i) & address != 0;
    }
    result
}
