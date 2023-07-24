use crate::memory::Memory;

pub struct CPU {
    memory: Memory,

}

impl CPU {
    
    pub fn new() -> Self {
        Self { 
            memory: Memory::new()
        }
    }

    pub fn load_rom(&mut self,data: Vec::<u8>) {
        let offset = 0x200;
        for (i, data) in data.iter().enumerate() {
            self.memory.write(offset + i, *data);
        }
    }



}