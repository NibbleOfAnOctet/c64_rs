mod cpu6510;
mod random_access_memory;

use cpu6510::CPU6510;

use self::random_access_memory::RandomAccessMemory;

pub struct Commodore{
    cpu6510: CPU6510,
    ram: RandomAccessMemory
}
impl Commodore{
    pub fn new()->Self{
        Self{
            cpu6510: CPU6510::new(),
            ram: RandomAccessMemory::new()
        }
    }

    pub fn reset(&mut self){
        self.cpu6510 = CPU6510::new();
        self.ram = RandomAccessMemory::new();
    }

    pub fn load_program(&mut self, bytes: Vec::<u8>){
        for i in 0..bytes.len(){
            self.ram.write_byte(i as u16, bytes[i]);
        }
    }

    pub fn run(&mut self){
        self.cpu6510.run(&mut self.ram);
    }
}