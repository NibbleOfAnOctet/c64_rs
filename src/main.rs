#![allow(dead_code)]
#![allow(unused_variables)]
mod commodore;

fn main() {
    let mut commodore = commodore::Commodore::new();
    let program = Vec::<u8>::from([0x01, 0x04, 0x00, 0x00, 0x00, 0x08, 0x00, 0x03, 0xDE]);

    commodore.load_program(program);
    commodore.run();
}
