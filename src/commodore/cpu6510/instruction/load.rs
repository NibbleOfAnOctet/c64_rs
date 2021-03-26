use super::{Implementation, Instruction};

pub fn get_instructions()->Vec<Instruction>{
    let mut instructions =  Vec::<Instruction>::new();
    let mnemonic = "LDA";
    instructions.push(Instruction{opcode:0xA9, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xA5, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0xB5, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xA1, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xB1, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 5;}))});//+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xAD, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xBD, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xB9, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 4;}))}); //+1 cycle if page boundary crossed
    let mnemonic = "STA";
    instructions.push(Instruction{opcode:0x85, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0x95, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0x81, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x91, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x8D, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0x9D, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x99, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 5;}))});
    let mnemonic = "LDX";
    instructions.push(Instruction{opcode:0xA2, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xA6, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0xB6, mnemonic, implementation: Some(Implementation::ZPY(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xAE, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xBE, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 4;}))}); //+1 cycle if page boundary crossed
    let mnemonic = "STX";
    instructions.push(Instruction{opcode:0x86, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0x96, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0x8E, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    let mnemonic = "LDY";
    instructions.push(Instruction{opcode:0xA0, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xA4, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0xB4, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xAC, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xBC, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 4;}))}); //+1 cycle if page boundary crossed
    let mnemonic = "STY";
    instructions.push(Instruction{opcode:0x84, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument|{return 3;}))});
    instructions.push(Instruction{opcode:0x94, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0x8C, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xAA, mnemonic:"TAX", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0x8A, mnemonic:"TXA", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0xA8, mnemonic:"TAY", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0x98, mnemonic:"TYA", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0xBA, mnemonic:"TSX", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0x9A, mnemonic:"TXS", implementation: Some(Implementation::Implied(|cpu|{return 2;}))});
    instructions.push(Instruction{opcode:0x68, mnemonic:"PLA", implementation: Some(Implementation::Implied(|cpu|{return 4;}))});
    instructions.push(Instruction{opcode:0x48, mnemonic:"PHA", implementation: Some(Implementation::Implied(|cpu|{return 3;}))});
    instructions.push(Instruction{opcode:0x28, mnemonic:"PLP", implementation: Some(Implementation::Implied(|cpu|{return 4;}))});
    instructions.push(Instruction{opcode:0x08, mnemonic:"PHP", implementation: Some(Implementation::Implied(|cpu|{return 3;}))});
    instructions
}
