use crate::bus::*;
use crate::constants::*;
use crate::cpu::*;

pub struct Emulator {
    cpu: Cpu,
    paused: bool,
}

impl Emulator {

    pub fn new() -> Emulator {
        let mut bus = Bus::new();
        bus.load_rom();

        Emulator {
            cpu: Cpu::new(bus),
            paused: false,
        }
    }

    pub fn run(&mut self) {
        let mut frame_cycles = 0;

        if !self.paused {
            while frame_cycles < MAX_CYCLES_PER_FRAME {
                let cycles = self.cpu.execute();
                frame_cycles += cycles as usize;

                // self.cpu.handle_interrupts();
            }
        }
    }
}