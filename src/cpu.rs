use crate::bus::*;
use crate::constants::*;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
struct RegisterPairParts {
    lo: Byte,
    hi: Byte,
}

union RegisterPair {
    val: Word,
    parts: RegisterPairParts,
}

// This is the Intel 8080 CPU
pub struct Cpu {
    // There are 7 general purpose registers - B C D E H L and Accumulator (A).
    // The registers are often referenced in pairs (i.e. BC) and A along with CPU Flags
    //     AF	-   A	F	Accumulator & Flags
    //     BC	-   B	C	BC (0, 1)
    //     DE	-   D	E	DE (2, 3)
    //     HL	-   H	L	HL (4, 5)
    //
    // There is a 2-Byte register for the Program counter and a 2-Byte register for the Stack Pointer
    af: RegisterPair,
    bc: RegisterPair,
    de: RegisterPair,
    hl: RegisterPair,

    program_counter: Word,
    stack_pointer: Word,

    bus: Bus,
}

impl Cpu {

    pub fn new(bus: Bus) -> Cpu {
        Cpu {
            af: RegisterPair { val: 0 },
            bc: RegisterPair { val: 0 },
            de: RegisterPair { val: 0 },
            hl: RegisterPair { val: 0 }.
            program_counter: 0,
            stack_pointer: 0,
            bus: bus
        }
    }

    pub fn debug(&self) {
    }

    fn read_memory(&self, addr: Word) -> Byte {
        self.bus.read_byte(addr)
    }

    fn write_memory(&mut self, addr: Word, data: Byte) {
        self.bus.write_byte(addr, data);
    }

    fn push_byte_to_stack(&mut self, data: Byte) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.write_memory(self.stack_pointer, data);
    }

    fn pop_byte_from_stack(&mut self) -> Byte {
        let data = self.read_memory(self.stack_pointer);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        data
    }

    fn push_word_to_stack(&mut self, data: Word) {
        let lo = data & 0xFF;
        let hi = data >> 8;
        self.push_byte_to_stack(hi as Byte);
        self.push_byte_to_stack(lo as Byte);
    }

    fn pop_word_from_stack(&mut self) -> Word {
        let lo = self.pop_byte_from_stack();
        let hi = self.pop_byte_from_stack();
        ((hi as Word) << 8) | lo as Word
    }
}