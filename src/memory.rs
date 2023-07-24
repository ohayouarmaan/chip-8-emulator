pub struct Memory {
    pub mem: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; 4096]
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        if address > 200 {
            self.mem[address] = value;
        }
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.mem[address]
    }
}