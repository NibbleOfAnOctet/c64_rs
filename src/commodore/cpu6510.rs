mod instruction;
use instruction::{Implementation, Instruction};
use super::random_access_memory::RandomAccessMemory;

pub enum CPUFlag{
    Negative    =0b10000000,
    Overflow    =0b01000000,
    Break       =0b00010000,
    DecimalMode =0b00001000,
    IRQDisable  =0b00000100,
    Zero        =0b00000010,
    Carry       =0b00000001
}
#[allow(non_snake_case)]
pub struct CPU6510 {
    opcode_table: [Instruction;256],
    A: u8, //Accumulator
    Y: u8, //Index register Y
    X: u8,   //Index register X
    PC: u16, //Program counter
    S: u16,  //Stack pointer
    P: u8, //Status register (Negative, Overflow, X, Break, Decimal mode, IRQ Disable, Zero, Carry)
}

impl CPU6510 {
    pub fn run(&mut self, ram: &mut RandomAccessMemory){
        loop{
            if self.flag_is_set(CPUFlag::Break) { println!("(Halted: Break flag set)"); break; }

            self.step(ram);
        }
    }

    pub fn set_flag_if(&mut self, flag: CPUFlag, on: bool){
        if on{
            self.set_flag(flag);
        }else{
            self.clear_flag(flag);
        }
    }
    pub fn set_flag(&mut self, flag: CPUFlag){
        self.P |= flag as u8;
    }

    pub fn clear_flag(&mut self, flag: CPUFlag){
        self.P &= !(flag as u8);
    }

    pub fn flag_is_set(&mut self, flag: CPUFlag)->bool{
        let flag = flag as u8;
        return (self.P & flag) == flag;
    }

    pub fn new() -> Self {
        let all_instructions = instruction::get_all_instructions();

        let mut opcode_table= [Instruction::default(); 256];
        
        for instruction in &all_instructions{
            opcode_table[instruction.opcode as usize] = *instruction;
        }

        Self {
            A: 0,
            Y: 0,
            X: 0,
            PC: 0,
            S: 0,
            P: 0,
            opcode_table: opcode_table
        }
    }

    pub fn step(&mut self, ram: &mut RandomAccessMemory) {
        let opcode = ram.read_byte(self.PC);
        let instruction = self.opcode_table[opcode as usize];
        
        match instruction.implementation {
            Some(Implementation::Implied(implementation)) => {
                let cycles = implementation(self);
                self.PC+=1;

                println!("{}", instruction.mnemonic);
            },
            Some(Implementation::Abs(implementation))=>{
                let argument= ram.read_word(self.PC+1);
                let value = ram.read_byte(argument);
                let cycles = implementation(self, value);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::AbsX(implementation))=>{
                let argument= ram.read_word(self.PC+1);
                let value = ram.read_byte(argument+self.X as u16);
                let cycles = implementation(self, value);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::AbsY(implementation))=>{
                let argument= ram.read_word(self.PC+1);
                let value = ram.read_byte(argument+self.Y as u16);
                let cycles = implementation(self, value);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::ZP(implementation))=>{
                let argument= ram.read_byte(self.PC+1);
                let value = ram.read_byte(argument as u16);
                let cycles = implementation(self, value);
                self.PC+=2;

                println!("{} ${:02X}", instruction.mnemonic, argument);
            },
            Some(Implementation::ZPX(implementation))=>{
                let argument= ram.read_byte(self.PC+1);
                let value = ram.read_byte((argument as u16).wrapping_add(self.X as u16));
                let cycles = implementation(self, value);
                self.PC+=2;

                println!("{} (${:02X},X)", instruction.mnemonic, argument);
            },
            Some(Implementation::ZPY(implementation))=>{
                let argument= ram.read_byte(self.PC+1);
                let value = ram.read_byte((argument as u16).wrapping_add(self.Y as u16));
                let cycles = implementation(self, value);
                self.PC+=2;

                println!("{} (${:02X},Y)", instruction.mnemonic, argument);
            },
            Some(Implementation::Indirect(implementation))=>{
                let argument = ram.read_word(self.PC+1);
                let value = ram.read_byte_indirect(argument);
                let cycles = implementation(self, value);
                self.PC+=3;

                println!("{} (${:04X})", instruction.mnemonic, argument);
            },
            Some(Implementation::IndirectZX(implementation))=>{
                let argument= ram.read_byte(self.PC+1);
                let value = ram.read_byte_indirect(argument.wrapping_add(self.X) as u16);
                let cycles = implementation(self, value);
                self.PC+=2;
                println!("{} (${:02X},X)", instruction.mnemonic, argument);
            },
            Some(Implementation::IndirectZY(implementation))=>{
                let argument = ram.read_byte(self.PC+1);
                let value = ram.read_byte_indirect((argument&0xff) as u16)+self.Y as u8;
                let cycles = implementation(self, argument);
                self.PC+=2;

                println!("{} (${:02X}),Y", instruction.mnemonic, argument);
            },
            Some(Implementation::Immediate(implementation)) => {
                let argument = ram.read_byte(self.PC+1);
                let cycles = implementation(self, argument);
                println!("{} #${:02X}", instruction.mnemonic, argument);
                self.PC+=2;
            },
            Some(Implementation::Relative(implementation)) => {
                let argument = ram.read_word(self.PC+1);
                let address = self.PC+argument;
                let value = ram.read_byte(address);
                let cycles = implementation(self, value);
                println!("{} ${:04X}", instruction.mnemonic, argument);
                self.PC+=3;
            },
            None=>{
                panic!("{:02X} Not implemented!", opcode);
            }
        }
    }

    
}
