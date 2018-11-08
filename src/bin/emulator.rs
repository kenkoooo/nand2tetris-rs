extern crate nand2tetris_rs;
extern crate termion;

use std::env;
use std::io::{stdin, stdout, Write};

use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

use nand2tetris_rs::computer::cpu::{CPUTrait, CPU};
use nand2tetris_rs::computer::memory::MemoryTrait;
use nand2tetris_rs::emulator::memory::EmulatedMemory;
use nand2tetris_rs::emulator::rom::ROM32K;
use nand2tetris_rs::{computer, emulator, tools};

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = tools::read_file(&args[1]).unwrap();
    let binary = tools::load_hack_binary(&content);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut rom = ROM32K::<EmulatedMemory>::new(&binary);
    let mut cpu = CPU::new();
    let mut memory = EmulatedMemory::new();

    let mut instruction = [false; 16];
    let mut in_m = [false; 16];
    let mut reset = true;
    for i in 0.. {
        cpu.tick(in_m, instruction, reset);
        let (out_m, write_m, address_m, pc) = cpu.tock(in_m, instruction);
        in_m = memory.access(out_m, write_m, address_m);
        instruction = rom.access(pc);

        // render
        for r in 0..256 {
            for c in 0..512 {
                let screen_address = convert_address(16384 + r * 32 + c / 16);
                let screen_word = memory.access([false; 16], false, screen_address);
                let is_black = screen_word[(c % 16) as usize];
                let content = if is_black { 'X' } else { '_' };
                write!(stdout, "{}{}", termion::cursor::Goto(r + 1, c + 1), content).unwrap();
            }
        }
        stdout.flush().unwrap();

        reset = false;
    }

    // for c in stdin.events() {
    //     let evt = c.unwrap();
    //     match evt {
    //         Event::Key(key) => match key {
    //             Key::Ctrl('c') => break,
    //             Key::Char(c) => {}
    //             _ => {
    //                 write!(stdout, "{:?}", key).unwrap();
    //             }
    //         },
    //         _ => {}
    //     }
    //     stdout.flush().unwrap();
    // }
}

fn convert_address(address: u16) -> [bool; 15] {
    let mut result = [false; 15];
    for i in 0..15 {
        result[i] = (1 << i) & address != 0;
    }
    result
}
