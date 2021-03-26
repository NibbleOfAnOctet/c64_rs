use super::{Implementation, Instruction};


pub fn get_instructions()->Vec<Instruction>{
    let mut instructions =  Vec::<Instruction>::new();
    let mnemonic = "SLO";
    instructions.push(Instruction{opcode:0x07, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x17, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x03, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x13, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x0F, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x1F, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0x1B, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    let mnemonic = "RLA";
    instructions.push(Instruction{opcode:0x27, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x37, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x23, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x33, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x2F, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x3F, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0x3B, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    let mnemonic = "SRE";
    instructions.push(Instruction{opcode:0x47, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x57, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x43, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x53, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x4F, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x5F, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0x5B, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    let mnemonic = "RRA";
    instructions.push(Instruction{opcode:0x67, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x77, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x63, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x73, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0x6F, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x7F, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0x7B, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    let mnemonic = "SAX";
    instructions.push(Instruction{opcode:0x87, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 3;}))});
    instructions.push(Instruction{opcode:0x97, mnemonic, implementation: Some(Implementation::ZPY(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0x83, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x8F, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    let mnemonic = "LAX";
    instructions.push(Instruction{opcode:0xA7, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 3;}))});
    instructions.push(Instruction{opcode:0xB7, mnemonic, implementation: Some(Implementation::ZPY(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xA3, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xB3, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 5;}))}); //+1 cycle if boundary crossed
    instructions.push(Instruction{opcode:0xAF, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 4;}))});
    instructions.push(Instruction{opcode:0xBF, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 4;}))}); //+1 cycle if boundary crossed
    let mnemonic = "DCP";
    instructions.push(Instruction{opcode:0xC7, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0xD7, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xC3, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0xD3, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0xCF, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xDF, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0xDB, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    let mnemonic = "ISC";
    instructions.push(Instruction{opcode:0xE7, mnemonic, implementation: Some(Implementation::ZP(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0xF7, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xE3, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0xF3, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument|{return 8;}))});
    instructions.push(Instruction{opcode:0xEF, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument|{return 6;}))});
    instructions.push(Instruction{opcode:0xFF, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0xFB, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 7;}))});
    instructions.push(Instruction{opcode:0x0B, mnemonic:"ANC", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0x2B, mnemonic:"ANC", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0x4B, mnemonic:"ALR", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0x6B, mnemonic:"ARR", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0x8B, mnemonic:"XAA", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xAB, mnemonic:"LAX", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xCB, mnemonic:"AXS", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    instructions.push(Instruction{opcode:0xEB, mnemonic:"SBC", implementation: Some(Implementation::Immediate(|cpu, argument|{return 2;}))});
    let mnemonic = "AHX";
    instructions.push(Instruction{opcode:0x93, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu, argument|{return 6;}))});
    instructions.push(Instruction{opcode:0x9F, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x9C, mnemonic:"SHY", implementation: Some(Implementation::AbsX(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x9E, mnemonic:"SHX", implementation: Some(Implementation::AbsY(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0x9B, mnemonic:"TAS", implementation: Some(Implementation::AbsY(|cpu, argument|{return 5;}))});
    instructions.push(Instruction{opcode:0xBB, mnemonic:"LAS", implementation: Some(Implementation::AbsY(|cpu, argument|{return 4;}))}); //+1 if boundary crossed
    
    //KIL
    let mnemonic = "KIL";
    for opcode in [0x02,0x12,0x22,0x32,0x42,0x52,0x62,0x72,0x92,0xB2,0xD2,0xF2].iter(){
        instructions.push(Instruction{opcode:*opcode, mnemonic, implementation: Some(Implementation::Implied(|cpu|{return 0;}))});
    }
    instructions
}
