const RAM_SIZE: usize = 0x10000;
pub struct RandomAccessMemory{
    bytes: [u8;RAM_SIZE]
}

impl RandomAccessMemory{
    pub fn new()->Self{
        Self{
            bytes:[0;RAM_SIZE]
        }
    }

    pub fn read_byte_indirect(&self, address:u16)->u8{
        return self.read_byte(self.read_word(address));
    }

    pub fn read_byte(&self, address:u16)->u8{
        return self.bytes[address as usize];
    }

    pub fn read_word(&self, address:u16)->u16{
        let value = ((self.bytes[(address+1) as usize] as u16) << 8) | self.bytes[address as usize] as u16;
        return value
    }

    pub fn write_word(&mut self, address: u16, value: u16){
        self.bytes[address as usize]=value as u8;
        self.bytes[(address+1) as usize]=((value&0xff00)>>8) as u8;
    }

    pub fn write_byte(&mut self, address: u16, value: u8){
        self.bytes[address as usize]=value;
    }
}