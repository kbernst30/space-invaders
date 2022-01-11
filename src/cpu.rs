use crate::bus::*;
use crate::constants::*;
use crate::ops::*;
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
            hl: RegisterPair { val: 0 },
            program_counter: 0,
            stack_pointer: 0,
            bus: bus
        }
    }

    pub fn debug(&self) {
    }

    pub fn execute(&mut self) -> u8 {
        // Execute operation and return cycles needed execute
        let op = self.read_memory(self.program_counter);
        let opcode = OPCODE_MAP
            .get(&op)
            .expect(&format!("OpCode 0x{:02x} is not recognized at PC - {:04X}", op, self.program_counter));

        self.program_counter = self.program_counter.wrapping_add(1);

        match opcode.operation {
            Operation::CMA => self.do_complement_accumulator(&opcode),
            Operation::CMC => self.do_complement_carry(&opcode),
            Operation::DAA => self.do_decimal_adjust_accumulator(&opcode),
            Operation::DCR => self.do_decrement(&opcode),
            Operation::INR => self.do_increment(&opcode),
            Operation::NOP => opcode.cycles,
            Operation::STC => self.do_set_carry(&opcode),
            _ => panic!("Unexpected opcode 0x{:02X} encountered", op),
        }
    }

    fn read_memory(&self, addr: Word) -> Byte {
        self.bus.read_byte(addr)
    }

    fn write_memory(&mut self, addr: Word, data: Byte) {
        self.bus.write_byte(addr, data);
    }

    fn get_next_byte(&mut self) -> Byte {
        let data = self.read_memory(self.program_counter);
        self.program_counter = self.program_counter.wrapping_add(1);
        data
    }

    fn get_next_word(&mut self) -> Word {
        let lo = self.get_next_byte();
        let hi = self.get_next_byte();
        ((hi as Word) << 8) | lo as Word
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

    fn is_zero_flag_set(&self) -> bool {
        unsafe {
            is_bit_set(&self.af.parts.lo, ZERO_FLAG)
        }
    }

    fn is_carry_flag_set(&self) -> bool {
        unsafe {
            is_bit_set(&self.af.parts.lo, CARRY_FLAG)
        }
    }

    fn is_sign_flag_set(&self) -> bool {
        unsafe {
            is_bit_set(&self.af.parts.lo, SIGN_FLAG)
        }
    }

    fn is_auxiliary_carry_flag_set(&self) -> bool {
        unsafe {
            is_bit_set(&self.af.parts.lo, AUXILIARY_CARRY_FLAG)
        }
    }

    fn is_parity_flag_set(&self) -> bool {
        unsafe {
            is_bit_set(&self.af.parts.lo, PARITY_FLAG)
        }
    }

    fn update_zero_flag(&mut self, val: bool) {
        unsafe {
            match val {
                true => set_bit(&mut self.af.parts.lo, ZERO_FLAG),
                false => reset_bit(&mut self.af.parts.lo, ZERO_FLAG)
            };
        }
    }

    fn update_carry_flag(&mut self, val: bool) {
        unsafe {
            match val {
                true => set_bit(&mut self.af.parts.lo, CARRY_FLAG),
                false => reset_bit(&mut self.af.parts.lo, CARRY_FLAG)
            };
        }
    }

    fn update_sign_flag(&mut self, val: bool) {
        unsafe {
            match val {
                true => set_bit(&mut self.af.parts.lo, SIGN_FLAG),
                false => reset_bit(&mut self.af.parts.lo, SIGN_FLAG)
            };
        }
    }

    fn update_auxiliary_carry_flag(&mut self, val: bool) {
        unsafe {
            match val {
                true => set_bit(&mut self.af.parts.lo, AUXILIARY_CARRY_FLAG),
                false => reset_bit(&mut self.af.parts.lo, AUXILIARY_CARRY_FLAG)
            };
        }
    }

    fn update_parity_flag(&mut self, val: bool) {
        unsafe {
            match val {
                true => set_bit(&mut self.af.parts.lo, PARITY_FLAG),
                false => reset_bit(&mut self.af.parts.lo, PARITY_FLAG)
            };
        }
    }

    fn do_complement_accumulator(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            self.af.parts.hi = !self.af.parts.hi;
            opcode.cycles
        }
    }

    fn do_complement_carry(&mut self, opcode: &OpCode) -> u8 {
        self.update_carry_flag(!self.is_carry_flag_set());
        opcode.cycles
    }

    fn do_decimal_adjust_accumulator(&mut self, opcode: &OpCode) -> u8 {
        // There is a description of how this algorithm works in the official
        // 8080 documentation/manual

        unsafe {
            let mut val = self.af.parts.hi;

            if self.is_auxiliary_carry_flag_set() || (val & 0xF) > 0x09 {
                self.update_auxiliary_carry_flag((val & 0xF) + 6 > 0xF);
                val = val.wrapping_add(6);
            }

            if self.is_carry_flag_set() || (val >> 4) > 0x09 {
                self.update_carry_flag(((val >> 4) + 6) > 0xF);
                let upper = ((val >> 4) + 6) & 0xF;
                let lower = val & 0xF;
                val = (upper << 4) | lower;
            }

            self.update_parity_flag(is_even_parity(&val));
            self.update_sign_flag(is_bit_set(&val, 7));
            self.update_zero_flag(val == 0);

            self.af.parts.hi = val;

            opcode.cycles
        }
    }

    fn do_decrement(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let res = match opcode.code {
                0x05 => { self.bc.parts.hi = self.bc.parts.hi.wrapping_sub(1); self.bc.parts.hi },
                0x0D => { self.bc.parts.lo = self.bc.parts.lo.wrapping_sub(1); self.bc.parts.lo },
                0x15 => { self.de.parts.hi = self.de.parts.hi.wrapping_sub(1); self.de.parts.hi },
                0x1D => { self.de.parts.lo = self.de.parts.lo.wrapping_sub(1); self.de.parts.lo },
                0x25 => { self.hl.parts.hi = self.hl.parts.hi.wrapping_sub(1); self.hl.parts.hi },
                0x2D => { self.hl.parts.lo = self.hl.parts.lo.wrapping_sub(1); self.hl.parts.lo },
                0x35 => {
                    let val = self.read_memory(self.hl.val).wrapping_sub(1);
                    self.write_memory(self.hl.val, val);
                    val
                },
                0x3D => { self.af.parts.hi = self.bc.parts.hi.wrapping_sub(1); self.af.parts.hi },
                _ => panic!("Unexpected code [{:02X}] encountered for DCR", opcode.code)
            };

            self.update_zero_flag(res == 0);
            self.update_sign_flag(is_bit_set(&res, 7));
            self.update_auxiliary_carry_flag(res & 0xF == 0xF);
            self.update_parity_flag(is_even_parity(&res));

            opcode.cycles
        }
    }

    fn do_increment(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let res = match opcode.code {
                0x04 => { self.bc.parts.hi = self.bc.parts.hi.wrapping_add(1); self.bc.parts.hi },
                0x0C => { self.bc.parts.lo = self.bc.parts.lo.wrapping_add(1); self.bc.parts.lo },
                0x14 => { self.de.parts.hi = self.de.parts.hi.wrapping_add(1); self.de.parts.hi },
                0x1C => { self.de.parts.lo = self.de.parts.lo.wrapping_add(1); self.de.parts.lo },
                0x24 => { self.hl.parts.hi = self.hl.parts.hi.wrapping_add(1); self.hl.parts.hi },
                0x2C => { self.hl.parts.lo = self.hl.parts.lo.wrapping_add(1); self.hl.parts.lo },
                0x34 => {
                    let val = self.read_memory(self.hl.val).wrapping_add(1);
                    self.write_memory(self.hl.val, val);
                    val
                },
                0x3C => { self.af.parts.hi = self.bc.parts.hi.wrapping_add(1); self.af.parts.hi },
                _ => panic!("Unexpected code [{:02X}] encountered for INR", opcode.code)
            };

            self.update_zero_flag(res == 0);
            self.update_sign_flag(is_bit_set(&res, 7));
            self.update_auxiliary_carry_flag(res & 0xF == 0);
            self.update_parity_flag(is_even_parity(&res));

            opcode.cycles
        }
    }

    fn do_set_carry(&mut self, opcode: &OpCode) -> u8 {
        self.update_carry_flag(true);
        opcode.cycles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_complement_accumulator() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x2F;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b10101010;
        cpu.do_complement_accumulator(&opcode);
        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b01010101);
        }
    }

    #[test]
    fn test_do_decimal_adjust_accumulator() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x27;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0x9B;
        cpu.do_decimal_adjust_accumulator(&opcode);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0x01);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_carry_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), false);
        }
    }

    #[test]
    fn test_do_decrement() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x05;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.bc.parts.hi = 0;
        cpu.do_decrement(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0xFF);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        cpu.bc.parts.hi = 1;
        cpu.do_decrement(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0);
            assert_eq!(cpu.is_zero_flag_set(), true);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        cpu.bc.parts.hi = 0x80;
        cpu.do_decrement(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0x7F);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), false);
        }

        cpu.bc.parts.hi = 0x7F;
        cpu.do_decrement(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0x7E);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

    #[test]
    fn test_do_increment() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x04;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.bc.parts.hi = 0;
        cpu.do_increment(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 1);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), false);
        }

        cpu.bc.parts.hi = 0xFF;
        cpu.do_increment(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0);
            assert_eq!(cpu.is_zero_flag_set(), true);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        cpu.bc.parts.hi = 0x7F;
        cpu.do_increment(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0x80);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), false);
        }

        cpu.bc.parts.hi = 0x80;
        cpu.do_increment(&opcode);
        unsafe {
            assert_eq!(cpu.bc.parts.hi, 0x81);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }
}