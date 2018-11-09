extern crate nand2tetris_rs;
extern crate termion;

use std::env;
use std::io::{stdout, Write};

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

use nand2tetris_rs::computer::cpu::CPU;
use nand2tetris_rs::emulator::emulator::Emulator;
use nand2tetris_rs::emulator::memory::EmulatedMemory;
use nand2tetris_rs::emulator::screen::Screen;
use nand2tetris_rs::tools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = tools::read_file(&args[1]).unwrap();
    let binary = tools::load_hack_binary(&content);

    let mut emulator = Emulator::<EmulatedMemory, CPU>::new(&binary);

    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    let mut screen = Screen::new();
    loop {
        emulator.iterate();

        // render
        let pixels = screen.update(&mut emulator.memory);
        for &(r, c) in pixels.iter() {
            let content = if screen.memory[r][c] { 'X' } else { ' ' };
            let r = r as u16;
            let c = c as u16;
            write!(stdout, "{}{}", termion::cursor::Goto(c + 1, r + 1), content).unwrap();
        }
        stdout.flush().unwrap();
    }
}
