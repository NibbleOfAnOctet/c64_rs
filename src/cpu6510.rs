mod instruction;
use super::random_access_memory::RandomAccessMemory;
use instruction::Instruction;
use instruction::Implementation;
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
    pub PC: u16, //Program counter
    S: u16,  //Stack pointer
    P: u8, //Status register (Negative, Overflow, X, Break, Decimal mode, IRQ Disable, Zero, Carry)
    pub ram: RandomAccessMemory
}

impl CPU6510 {

    pub fn not_implemented(&mut self){
        print!("NOT IMPLEMENTED: ");
        //self.set_flag(CPUFlag::Break);
    }

    pub fn load_program(&mut self, bytes: Vec<u8>){
        for i in 0..bytes.len(){
            self.ram.write_byte(i as u16, bytes[i]);
        }
    }
    pub fn run(&mut self){
        loop{
            if self.flag_is_set(CPUFlag::Break) { println!("(Halted: Break flag set)"); break; }

            self.step();
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
            opcode_table: opcode_table,
            ram:RandomAccessMemory::new()
        }
    }

    pub fn step(&mut self) {
        let opcode = self.ram.read_byte(self.PC);
        let instruction = self.opcode_table[opcode as usize];
        print!("{:#04X}: ", self.PC);
        match instruction.implementation {
            Some(Implementation::Implied(implementation)) => {
                let cycles = implementation(self);
                self.PC+=1;

                println!("{}", instruction.mnemonic);
            },
            Some(Implementation::Abs(implementation))=>{
                let argument= self.ram.read_word(self.PC+1);
                let value = self.ram.read_byte(argument);
                let cycles = implementation(self, value, argument);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::AbsX(implementation))=>{
                let argument= self.ram.read_word(self.PC+1);
                let address = argument+self.X as u16;
                let value = self.ram.read_byte(address);
                let cycles = implementation(self, value, argument);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::AbsY(implementation))=>{
                let argument= self.ram.read_word(self.PC+1);
                let address = argument+self.Y as u16;
                let value = self.ram.read_byte(address);
                let cycles = implementation(self, value,address);
                self.PC+=3;

                println!("{} ${:04X}", instruction.mnemonic, argument);
            },
            Some(Implementation::ZP(implementation))=>{
                let argument= self.ram.read_byte(self.PC+1);
                let value = self.ram.read_byte(argument as u16);
                let cycles = implementation(self, value, argument as u16);
                self.PC+=2;

                println!("{} ${:02X}", instruction.mnemonic, argument);
            },
            Some(Implementation::ZPX(implementation))=>{
                let argument= self.ram.read_byte(self.PC+1);
                let address = (argument as u16).wrapping_add(self.X as u16);
                let value = self.ram.read_byte(address);
                let cycles = implementation(self, value, address);
                self.PC+=2;

                println!("{} (${:02X},X)", instruction.mnemonic, argument);
            },
            Some(Implementation::ZPY(implementation))=>{
                let argument= self.ram.read_byte(self.PC+1);
                let address = (argument as u16).wrapping_add(self.Y as u16);
                let value = self.ram.read_byte(address);
                let cycles = implementation(self, value, address);
                self.PC+=2;

                println!("{} (${:02X},Y)", instruction.mnemonic, argument);
            },
            Some(Implementation::Indirect(implementation))=>{
                let argument = self.ram.read_word(self.PC+1);
                let value = self.ram.read_byte_indirect(argument);
                let cycles = implementation(self, value, argument);
                self.PC+=3;

                println!("{} (${:04X})", instruction.mnemonic, argument);
            },
            Some(Implementation::IndirectZX(implementation))=>{
                let argument= self.ram.read_byte(self.PC+1);
                let address=argument.wrapping_add(self.X) as u16;
                let value = self.ram.read_byte_indirect(address);
                let cycles = implementation(self, value, address);
                self.PC+=2;
                println!("{} (${:02X},X)", instruction.mnemonic, argument);
            },
            Some(Implementation::IndirectZY(implementation))=>{
                let argument = self.ram.read_byte(self.PC+1);
                let address= (argument&0xff) as u16;
                let value = self.ram.read_byte_indirect(address)+self.Y as u8;
                let cycles = implementation(self, argument, address);
                self.PC+=2;

                println!("{} (${:02X}),Y", instruction.mnemonic, argument);
            },
            Some(Implementation::Immediate(implementation)) => {
                let argument = self.ram.read_byte(self.PC+1);
                let cycles = implementation(self, argument);
                println!("{} #${:02X}", instruction.mnemonic, argument);
                self.PC+=2;
            },
            Some(Implementation::Relative(implementation)) => {
                let argument = self.ram.read_byte(self.PC+1);

                let address = match (argument&0x80)>>7==1{
                    true=>self.PC.wrapping_sub((!argument&0x7f) as u16) + 1,
                    false=>self.PC.wrapping_add((argument&0x7f) as u16) + 2
                };

                let value = self.ram.read_byte(address+2);
                let cycles = implementation(self, value, address);

                println!("{} ${:04X}", instruction.mnemonic, address);
                self.PC+=2;
            },
            None=>{
                panic!("{:02X} Not implemented!", opcode);
            }
        }
    }

    
}
