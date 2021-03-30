use super::{Implementation, Instruction};

pub fn get_instructions()->Vec<Instruction>{
    let mut instructions =  Vec::<Instruction>::new();
    //Branch (Relative)
    instructions.push(Instruction{opcode:0x10, mnemonic:"BPL", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0x30, mnemonic:"BMI", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0x50, mnemonic:"BVC", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0x70, mnemonic:"BVS", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0x90, mnemonic:"BCC", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0xB0, mnemonic:"BCS", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0xD0, mnemonic:"BNE", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0xF0, mnemonic:"BEQ", implementation: Some(Implementation::Relative(|cpu,argument, address|{cpu.not_implemented();return 2;}))}); //+1 cycle if branch taken
    instructions.push(Instruction{opcode:0x40, mnemonic:"RTI", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x20, mnemonic:"JSR", implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x60, mnemonic:"RTS", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 6;}))});
    let mnemonic = "JMP";
    instructions.push(Instruction{opcode:0x4C, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 3;}))});
    instructions.push(Instruction{opcode:0x6C, mnemonic, implementation: Some(Implementation::Indirect(|cpu,argument, address|{cpu.not_implemented();return 5;}))});
    let mnemonic = "BIT";
    instructions.push(Instruction{opcode:0x24, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{cpu.not_implemented();return 3;}))});
    instructions.push(Instruction{opcode:0x2C, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 4;}))});
    instructions.push(Instruction{opcode:0x18, mnemonic:"CLC", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x38, mnemonic:"SEC", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0xD8, mnemonic:"CLD", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0xF8, mnemonic:"SED", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x58, mnemonic:"CLI", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x78, mnemonic:"SEI", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0xB8, mnemonic:"CLV", implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions
}
