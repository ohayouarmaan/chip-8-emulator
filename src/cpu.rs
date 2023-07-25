use crate::memory::{Memory, self};
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

        
        c.load_font();
        
        for i in 0x0..=0xF{
            c.registers.insert(format!("V{}", format!("{:x}", i).to_ascii_uppercase()), 0);
        }

        return c;
    }

    pub fn load_font(&mut self) {
        // Character 0
        self.memory.mem[0] = 0xF0;
        self.memory.mem[1] = 0x90;
        self.memory.mem[2] = 0x90;
        self.memory.mem[3] = 0x90;
        self.memory.mem[4] = 0xF0;
    
        // Character 1
        self.memory.mem[5] = 0x20;
        self.memory.mem[6] = 0x60;
        self.memory.mem[7] = 0x20;
        self.memory.mem[8] = 0x20;
        self.memory.mem[9] = 0x70;

        // Character 2
        self.memory.mem[10] = 0xF0;
        self.memory.mem[11] = 0x10;
        self.memory.mem[12] = 0xF0;
        self.memory.mem[13] = 0x80;
        self.memory.mem[14] = 0xF0;

        // Character 3
        self.memory.mem[15] = 0xF0;
        self.memory.mem[16] = 0x10;
        self.memory.mem[17] = 0xF0;
        self.memory.mem[18] = 0x10;
        self.memory.mem[19] = 0xF0;

        // Character 4
        self.memory.mem[20] = 0x90;
        self.memory.mem[21] = 0x90;
        self.memory.mem[22] = 0xF0;
        self.memory.mem[23] = 0x10;
        self.memory.mem[24] = 0x10;

        // Character 5
        self.memory.mem[25] = 0xF0;
        self.memory.mem[26] = 0x80;
        self.memory.mem[27] = 0xF0;
        self.memory.mem[28] = 0x10;
        self.memory.mem[29] = 0xF0;

        // Character 6
        self.memory.mem[30] = 0xF0;
        self.memory.mem[31] = 0x80;
        self.memory.mem[32] = 0xF0;
        self.memory.mem[33] = 0x90;
        self.memory.mem[34] = 0xF0;

        // Character 7
        self.memory.mem[35] = 0xF0;
        self.memory.mem[36] = 0x10;
        self.memory.mem[37] = 0x20;
        self.memory.mem[38] = 0x40;
        self.memory.mem[39] = 0x40;

        // Character 8
        self.memory.mem[40] = 0xF0;
        self.memory.mem[41] = 0x90;
        self.memory.mem[42] = 0xF0;
        self.memory.mem[43] = 0x90;
        self.memory.mem[44] = 0xF0;

        // Character 9
        self.memory.mem[45] = 0xF0;
        self.memory.mem[46] = 0x90;
        self.memory.mem[47] = 0xF0;
        self.memory.mem[48] = 0x10;
        self.memory.mem[49] = 0xF0;

        // Character A
        self.memory.mem[50] = 0xF0;
        self.memory.mem[51] = 0x90;
        self.memory.mem[52] = 0xF0;
        self.memory.mem[53] = 0x90;
        self.memory.mem[54] = 0x90;

        // Character B
        self.memory.mem[55] = 0xE0;
        self.memory.mem[56] = 0x90;
        self.memory.mem[57] = 0xE0;
        self.memory.mem[58] = 0x90;
        self.memory.mem[59] = 0xE0;

        // Character C
        self.memory.mem[60] = 0xF0;
        self.memory.mem[61] = 0x80;
        self.memory.mem[62] = 0x80;
        self.memory.mem[63] = 0x80;
        self.memory.mem[64] = 0xF0;

        // Character D
        self.memory.mem[65] = 0xE0;
        self.memory.mem[66] = 0x90;
        self.memory.mem[67] = 0x90;
        self.memory.mem[68] = 0x90;
        self.memory.mem[69] = 0xE0;

        // Character E
        self.memory.mem[70] = 0xF0;
        self.memory.mem[71] = 0x80;
        self.memory.mem[72] = 0xF0;
        self.memory.mem[73] = 0x80;
        self.memory.mem[74] = 0xF0;

        // Character F
        self.memory.mem[75] = 0xF0;
        self.memory.mem[76] = 0x80;
        self.memory.mem[77] = 0xF0;
        self.memory.mem[78] = 0x80;
        self.memory.mem[79] = 0x80;
    }

    pub fn load_rom(&mut self,data: Vec::<u8>) {
        let offset = 0x200;
        for (i, data) in data.iter().enumerate() {
            self.memory.write(offset + i, *data);
        };
    }

}