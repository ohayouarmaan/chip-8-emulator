use crate::memory::Memory;
use std::collections::HashMap;

pub struct CPU {
    pub memory: Memory,
    pub registers: HashMap<String, u8>,
}

impl CPU {
    
    pub fn new() -> Self {
        let mut c = Self { 
            memory: Memory::new(),
            registers: HashMap::new(),
        };

        
        for i in 0x0..=0xF{
            c.registers.insert(format!("V{}", format!("{:x}", i).to_ascii_uppercase()), 0);
        }

        return c;
    }

    pub fn load_rom(&mut self,data: Vec::<u8>) {
        let offset = 0x200;
        for (i, data) in data.iter().enumerate() {
            self.memory.write(offset + i, *data);
        }
    }
}