use crate::cpu::*;

pub struct Emulator {
    cpu: Cpu
}

impl Emulator {

    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(Bus::new())
        }
    }

    pub fn run(&self) {
        self.cpu.debug();
    }
}