pub mod arithmetic;
pub mod illegal;
pub mod jump;
pub mod load;
pub mod other;
use super::CPU6510;

#[derive(Default, Clone, Copy)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub opcode: u8,
    pub implementation: Option<Implementation>,
}

pub fn get_all_instructions() -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut bitwise = arithmetic::get_instructions();
    let mut other = other::get_instructions();
    let mut load = load::get_instructions();
    let mut jump = jump::get_instructions();
    let mut illegal = illegal::get_instructions();

    instructions.append(&mut bitwise);
    instructions.append(&mut other);
    instructions.append(&mut load);
    instructions.append(&mut jump);
    instructions.append(&mut illegal);

    return instructions;
}

#[derive(Clone, Copy)]
pub enum Implementation {
    Implied(fn(&mut CPU6510) -> u8),
    Immediate(fn(&mut CPU6510, u8) -> u8),  //#$00
    ZP(fn(&mut CPU6510, u8, u16) -> u8),         //$00
    ZPX(fn(&mut CPU6510, u8, u16) -> u8),        //$00,X
    ZPY(fn(&mut CPU6510, u8, u16) -> u8),        //$00,Y
    Indirect(fn(&mut CPU6510, u8, u16) -> u8),   //($0000)
    IndirectZX(fn(&mut CPU6510, u8, u16) -> u8), //($00),X
    IndirectZY(fn(&mut CPU6510, u8, u16) -> u8), //($00,Y)
    Abs(fn(&mut CPU6510, u8, u16) -> u8),        //$0000
    AbsX(fn(&mut CPU6510, u8, u16) -> u8),       //$0000, X
    AbsY(fn(&mut CPU6510, u8, u16) -> u8),       //$0000, Y
    Relative(fn(&mut CPU6510, u8, u16) -> u8),   //PC+$0000
}
