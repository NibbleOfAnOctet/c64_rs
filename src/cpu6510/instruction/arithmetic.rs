use crate::cpu6510::CPUFlag;
use super::{CPU6510, Implementation, Instruction};

const SIGN_BIT:u8 = 0b10000000;


fn bitwise_or(cpu: &mut CPU6510, value:u8){
    cpu.A|=value;

    cpu.set_flag_if(CPUFlag::Zero, cpu.A==0);
    cpu.set_flag_if(CPUFlag::Zero, is_negative(cpu.A));
}

fn bitwise_and(cpu: &mut CPU6510, value:u8){
    cpu.A&=value;
    
    cpu.set_flag_if(CPUFlag::Zero, cpu.A==0);
    cpu.set_flag_if(CPUFlag::Zero, is_negative(cpu.A));
}

fn bitwise_xor(cpu: &mut CPU6510, value:u8){
    cpu.A^=value;

    cpu.set_flag_if(CPUFlag::Zero, cpu.A==0);
    cpu.set_flag_if(CPUFlag::Zero, is_negative(cpu.A));
}

/**
Todo: Decimal mode
*/
fn add_with_carry(cpu: &mut CPU6510, value:u8){
    let was_negative = is_negative(cpu.A);
    let result = cpu.A as u16 + value as u16 + cpu.flag_is_set(CPUFlag::Carry) as u16;

    cpu.A = (result as u8)&0xff;

    let is_negative = is_negative(cpu.A);
    let overflowed = is_negative!= was_negative;

    cpu.set_flag_if(CPUFlag::Overflow,overflowed);
    cpu.set_flag_if(CPUFlag::Zero, cpu.A==0);
    cpu.set_flag_if(CPUFlag::Negative, is_negative);
    cpu.set_flag_if(CPUFlag::Carry, result>0xff);
}

/**
Todo: Decimal mode
*/
fn subtract_with_borrow(cpu: &mut CPU6510, value:u8){
    let was_negative = is_negative(cpu.A);
    let new_carry = value<cpu.A.wrapping_sub(!cpu.flag_is_set(CPUFlag::Carry) as u8);
    cpu.A = cpu.A.wrapping_sub(!cpu.flag_is_set(CPUFlag::Carry) as u8).wrapping_sub(value as u8).wrapping_sub(!new_carry as u8);
    let is_negative = is_negative(cpu.A);
    let overflowed = is_negative!= was_negative;
    cpu.set_flag_if(CPUFlag::Overflow,overflowed);
    cpu.set_flag_if(CPUFlag::Zero, cpu.A==0);
    cpu.set_flag_if(CPUFlag::Negative, is_negative);
    cpu.set_flag_if(CPUFlag::Carry, new_carry);
}


fn compare(cpu: &mut CPU6510, val1: u8, val2:u8){
    cpu.set_flag_if(CPUFlag::Zero, val1==val2);
    cpu.set_flag_if(CPUFlag::Negative, val1<val2);
    cpu.set_flag_if(CPUFlag::Carry, val1>val2);
}

fn decrement_with_negzero_flags(cpu: &mut CPU6510, value:u8)->u8{
    let newval = value.wrapping_sub(1);
    cpu.set_flag_if(CPUFlag::Negative, is_negative(newval));
    cpu.set_flag_if(CPUFlag::Negative, newval==0);

    value
}

fn increment_with_negzero_flags(cpu: &mut CPU6510, value:u8)->u8{
    let newval = value.wrapping_add(1);
    cpu.set_flag_if(CPUFlag::Negative, is_negative(newval));
    cpu.set_flag_if(CPUFlag::Negative, newval==0);

    value
}

fn decrement_at_address(cpu: &mut CPU6510, address:u16){
    let newval = cpu.ram.read_byte(address).wrapping_sub(1);
    cpu.ram.write_byte(address, newval);
    cpu.set_flag_if(CPUFlag::Negative, is_negative(newval));
    cpu.set_flag_if(CPUFlag::Zero, newval==0);
}

fn increment_at_address(cpu: &mut CPU6510, address:u16){
    let newval = cpu.ram.read_byte(address).wrapping_add(1);
    cpu.ram.write_byte(address, newval);
    cpu.set_flag_if(CPUFlag::Negative, is_negative(newval));
    cpu.set_flag_if(CPUFlag::Negative, newval==0);
}



fn is_negative(number: u8)->bool{
    number&SIGN_BIT==SIGN_BIT
}

pub fn get_instructions()->Vec<Instruction>{
    let mut instructions =  Vec::<Instruction>::new();
    let mnemonic = "ORA";
    instructions.push(Instruction{opcode:0x01, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{bitwise_or(cpu,argument);return 6;}))});
    instructions.push(Instruction{opcode:0x05, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{bitwise_or(cpu,argument);return 3;}))});
    instructions.push(Instruction{opcode:0x09, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{bitwise_or(cpu,argument);return 2;}))});
    instructions.push(Instruction{opcode:0x0D, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{bitwise_or(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x11, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{bitwise_or(cpu,argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x15, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{bitwise_or(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x19, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{bitwise_or(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x1D, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{bitwise_or(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "AND";
    instructions.push(Instruction{opcode:0x21, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{bitwise_and(cpu,argument);return 6;}))});
    instructions.push(Instruction{opcode:0x25, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{bitwise_and(cpu,argument);return 3;}))});
    instructions.push(Instruction{opcode:0x29, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{bitwise_and(cpu,argument);return 2;}))});
    instructions.push(Instruction{opcode:0x2D, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{bitwise_and(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x31, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{bitwise_and(cpu,argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x35, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{bitwise_and(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x39, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{bitwise_and(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x3D, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{bitwise_and(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "EOR";
    instructions.push(Instruction{opcode:0x49, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{bitwise_xor(cpu,argument);return 2;}))});
    instructions.push(Instruction{opcode:0x45, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{bitwise_xor(cpu,argument);return 3;}))});
    instructions.push(Instruction{opcode:0x55, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{bitwise_xor(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x41, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{bitwise_xor(cpu,argument);return 6;}))});
    instructions.push(Instruction{opcode:0x51, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{bitwise_xor(cpu,argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x4D, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{bitwise_xor(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x5D, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{bitwise_xor(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x59, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{bitwise_xor(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "ADC";
    instructions.push(Instruction{opcode:0x69, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{add_with_carry(cpu,argument);return 2;}))});
    instructions.push(Instruction{opcode:0x65, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{add_with_carry(cpu,argument);return 3;}))});
    instructions.push(Instruction{opcode:0x75, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{add_with_carry(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x61, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{add_with_carry(cpu,argument);return 6;}))});
    instructions.push(Instruction{opcode:0x71, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{add_with_carry(cpu,argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x6D, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{add_with_carry(cpu,argument);return 4;}))});
    instructions.push(Instruction{opcode:0x7D, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{add_with_carry(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0x79, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{add_with_carry(cpu,argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "SBC";
    instructions.push(Instruction{opcode:0xE9, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{subtract_with_borrow(cpu, argument);return 2;}))});
    instructions.push(Instruction{opcode:0xE5, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 3;}))});
    instructions.push(Instruction{opcode:0xF5, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 4;}))});
    instructions.push(Instruction{opcode:0xE1, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 6;}))});
    instructions.push(Instruction{opcode:0xF1, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xED, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 4;}))});
    instructions.push(Instruction{opcode:0xFD, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xF9, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{subtract_with_borrow(cpu, argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "CMP";
    instructions.push(Instruction{opcode:0xC9, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{compare(cpu, cpu.A,argument);return 2;}))});
    instructions.push(Instruction{opcode:0xC5, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 3;}))});
    instructions.push(Instruction{opcode:0xD5, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 4;}))});
    instructions.push(Instruction{opcode:0xC1, mnemonic, implementation: Some(Implementation::IndirectZX(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 6;}))});
    instructions.push(Instruction{opcode:0xD1, mnemonic, implementation: Some(Implementation::IndirectZY(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 5;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xCD, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 4;}))});
    instructions.push(Instruction{opcode:0xDD, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 4;}))}); //+1 cycle if page boundary crossed
    instructions.push(Instruction{opcode:0xD9, mnemonic, implementation: Some(Implementation::AbsY(|cpu,argument, address|{compare(cpu, cpu.A,argument);return 4;}))}); //+1 cycle if page boundary crossed
    
    let mnemonic = "CPX";
    instructions.push(Instruction{opcode:0xE0, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{compare(cpu,cpu.X, argument);return 2;}))});
    instructions.push(Instruction{opcode:0xE4, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{compare(cpu,cpu.X, argument);return 3;}))});
    instructions.push(Instruction{opcode:0xEC, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{compare(cpu,cpu.X, argument);return 4;}))});
    
    let mnemonic = "CPY";
    instructions.push(Instruction{opcode:0xC0, mnemonic, implementation: Some(Implementation::Immediate(|cpu,argument|{compare(cpu,cpu.Y, argument);return 2;}))});
    instructions.push(Instruction{opcode:0xC4, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{compare(cpu,cpu.Y, argument);return 3;}))});
    instructions.push(Instruction{opcode:0xCC, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{compare(cpu,cpu.Y, argument);return 4;}))});
    
    let mnemonic = "DEC";
    instructions.push(Instruction{opcode:0xC6, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{decrement_at_address(cpu, address);return 5;}))});
    instructions.push(Instruction{opcode:0xD6, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{decrement_at_address(cpu, address);return 6;}))});
    instructions.push(Instruction{opcode:0xCE, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{decrement_at_address(cpu, address);return 6;}))});
    instructions.push(Instruction{opcode:0xDE, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{decrement_at_address(cpu, address);return 7;}))});
    
    let mnemonic = "DEX";
    instructions.push(Instruction{opcode:0xCA, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.X= decrement_with_negzero_flags(cpu,cpu.X); return 2;}))});
    
    let mnemonic = "DEY";
    instructions.push(Instruction{opcode:0x88, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.Y= decrement_with_negzero_flags(cpu,cpu.Y); return 2;}))});
    
    let mnemonic = "INC";
    instructions.push(Instruction{opcode:0xE6, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{increment_at_address(cpu, address); return 5;}))});
    instructions.push(Instruction{opcode:0xF6, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{increment_at_address(cpu, address); return 6;}))});
    instructions.push(Instruction{opcode:0xEE, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{increment_at_address(cpu, address); return 6;}))});
    instructions.push(Instruction{opcode:0xFE, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{increment_at_address(cpu, address); return 7;}))});
    
    let mnemonic = "INX";
    instructions.push(Instruction{opcode:0xE8, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.X= increment_with_negzero_flags(cpu,cpu.X);return 2;}))});
    
    let mnemonic = "INY";
    instructions.push(Instruction{opcode:0xC8, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.Y= increment_with_negzero_flags(cpu,cpu.Y);return 2;}))});
    
    let mnemonic = "ASL";
    instructions.push(Instruction{opcode:0x0A, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x06, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{cpu.not_implemented();return 5;}))});
    instructions.push(Instruction{opcode:0x16, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x0E, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x1E, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{cpu.not_implemented();return 7;}))});
    
    let mnemonic = "ROL";
    instructions.push(Instruction{opcode:0x2A, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x26, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{cpu.not_implemented();return 5;}))});
    instructions.push(Instruction{opcode:0x36, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x2E, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x3E, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{cpu.not_implemented();return 7;}))});
    
    let mnemonic = "LSR";
    instructions.push(Instruction{opcode:0x4A, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x46, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{cpu.not_implemented();return 5;}))});
    instructions.push(Instruction{opcode:0x56, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x4E, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x5E, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{cpu.not_implemented();return 7;}))});
    
    let mnemonic = "ROR";
    instructions.push(Instruction{opcode:0x6A, mnemonic, implementation: Some(Implementation::Implied(|cpu|{cpu.not_implemented();return 2;}))});
    instructions.push(Instruction{opcode:0x66, mnemonic, implementation: Some(Implementation::ZP(|cpu,argument, address|{cpu.not_implemented();return 5;}))});
    instructions.push(Instruction{opcode:0x76, mnemonic, implementation: Some(Implementation::ZPX(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x6E, mnemonic, implementation: Some(Implementation::Abs(|cpu,argument, address|{cpu.not_implemented();return 6;}))});
    instructions.push(Instruction{opcode:0x7E, mnemonic, implementation: Some(Implementation::AbsX(|cpu,argument, address|{cpu.not_implemented();return 7;}))});
    instructions
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_with_carry_overflow_bit_test(){
        let mut cpu = CPU6510::new();

        //Overflow
        cpu.A=0x7F;
        add_with_carry(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0x7F+1);
        assert!(cpu.flag_is_set(CPUFlag::Overflow));
        assert!(cpu.flag_is_set(CPUFlag::Negative));

        //Overflow
        cpu.A=0xFF;
        add_with_carry(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0x00);
        assert!(cpu.flag_is_set(CPUFlag::Overflow));
        assert!(!cpu.flag_is_set(CPUFlag::Negative));
    }

    #[test]
    fn add_with_carry_adds_test(){   
        let mut cpu = CPU6510::new();
        cpu.A=0x10;
        add_with_carry(&mut cpu, 0x5);
        assert_eq!(cpu.A, 0x15);
        assert!(!cpu.flag_is_set(CPUFlag::Overflow));
        assert!(!cpu.flag_is_set(CPUFlag::Negative));
    }
    #[test]
    fn add_with_carry_sets_carry_flag_test(){
        let mut cpu = CPU6510::new();
        cpu.A=0xFF;
        add_with_carry(&mut cpu, 0x1);
        assert!(cpu.flag_is_set(CPUFlag::Carry));
    }

    #[test]
    fn add_with_carry_clears_negative_flag_test(){
        let mut cpu = CPU6510::new();
        cpu.A=0xFF;
        add_with_carry(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0x00);
        assert!(cpu.flag_is_set(CPUFlag::Overflow));
        assert!(cpu.flag_is_set(CPUFlag::Zero));
        assert!(!cpu.flag_is_set(CPUFlag::Negative));
    }

    #[test]
    fn subtract_with_borrow_test(){
        let mut cpu = CPU6510::new();
        cpu.A=0x5;
        cpu.set_flag_if(CPUFlag::Carry,true);
        subtract_with_borrow(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0x04);
        assert!(cpu.flag_is_set(CPUFlag::Carry));

        cpu.A=0x5;
        cpu.set_flag_if(CPUFlag::Carry,false);
        subtract_with_borrow(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0x03);
        assert!(cpu.flag_is_set(CPUFlag::Carry));


        cpu.A=0x0;
        cpu.set_flag_if(CPUFlag::Carry,true);
        subtract_with_borrow(&mut cpu, 0x1);
        assert_eq!(cpu.A, 0xfe);
        assert!(!cpu.flag_is_set(CPUFlag::Carry));
        assert!(cpu.flag_is_set(CPUFlag::Overflow));
        
    }
}