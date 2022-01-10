use crate::constants::*;

pub struct Bus {
    memory: [Byte; MEMORY_SIZE]
}

impl Bus {

    pub fn new() -> Bus {
        Bus {
            memory: [0; MEMORY_SIZE]
        }
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: Word, data: Byte) {
        self.memory[addr as usize] = data;
    }
}
