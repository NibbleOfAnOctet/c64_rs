use crate::cpu6510::CPUFlag;
use super::{Implementation, Instruction};

pub fn get_instructions()->Vec<Instruction>{
    let mut instructions =  Vec::<Instruction>::new();
    let mnemonic = "BRK";
    instructions.push(Instruction{opcode:0x00, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.set_flag(CPUFlag::Break); return 7;}))});
    //NOP
    let mnemonic = "NOP";
    for opcode in [0x04,0x44,0x64].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::ZP(|cpu,value, address|{cpu.not_implemented();return 3;}))});
    }
    for opcode in [0x0C].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::Abs(|cpu,value, address|{cpu.not_implemented();return 4;}))});
    }
    for opcode in [0x14,0x34,0x54,0x74,0xd4,0xf4].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::ZPX(|cpu,value, address|{cpu.not_implemented();return 4;}))});
    }
    for opcode in [0x1A,0x3A,0x5A,0x7A,0xDA,0xEA,0xFA].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    }
    for opcode in [0x1C,0x3C,0x5C,0x7C,0xDC,0xFC].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::AbsX(|cpu,value, address|{cpu.not_implemented();return 4;}))}); //+1 cycle if page boundary crossed
    }
    for opcode in [0x80,0x82,0x89,0xC2,0xE2].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::Immediate(|cpu,value|{cpu.not_implemented();return 2;}))}); //+1 cycle if page boundary crossed
    }
    instructions
}
