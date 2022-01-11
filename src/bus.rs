use std::fs;

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

    pub fn load_rom(&mut self) {
        // We will hardcode file names (at least for now) as the
        // emulated hardware is specific to Space Invaders
        // According to documentation, the 4 rom files should be loaded
        // into memory as follows:
        //   invaders.h 0000-07FF
        //   invaders.g 0800-0FFF
        //   invaders.f 1000-17FF
        //   invaders.e 1800-1FFF

        let invaders_h = fs::read("rom/invaders.h")
            .expect("Something went wrong reading the file");

        let invaders_g = fs::read("rom/invaders.g")
            .expect("Something went wrong reading the file");

        let invaders_f = fs::read("rom/invaders.f")
            .expect("Something went wrong reading the file");

        let invaders_e = fs::read("rom/invaders.e")
            .expect("Something went wrong reading the file");

        for i in 0x0000..0x2000 {
            match i {
                0x0000..=0x07FF => self.memory[i] = invaders_h[i],
                0x0800..=0x0FFF => self.memory[i] = invaders_h[i - 0x0800],
                0x1000..=0x17FF => self.memory[i] = invaders_h[i - 0x1000],
                0x1800..=0x1FFF => self.memory[i] = invaders_h[i - 0x1800],
                _ => println!("Tried to load bad ROM address {:04X}", i)
            };
        }
    }

    pub fn read_byte(&self, addr: Word) -> Byte {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: Word, data: Byte) {
        self.memory[addr as usize] = data;
    }
}
