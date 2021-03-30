#![allow(dead_code)]
#![allow(unused_variables)]
mod random_access_memory;
mod cpu6510;
use std::fs;

use cpu6510::CPU6510;

fn main() {
    let mut commodore = CPU6510::new();
    let rom = fs::read("kernal.bin").unwrap();
    for i in 0..rom.len(){
        commodore.ram.write_byte((0xe000+i) as u16, rom[i as usize]);
    }
    commodore.PC = 0xe000;
    commodore.run();
}
