use std::io::prelude::*;
use std::fs::File;
use memory::Memory;
use cpu::CPU;

mod memory;
mod cpu;

fn main() {
    let mut f = File::open("./src/data/Breakout.ch8").unwrap();
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer).unwrap();
    let mut chip8 = CPU::new();
    chip8.load_rom(buffer);
}
