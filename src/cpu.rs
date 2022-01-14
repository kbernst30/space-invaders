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
            Operation::ADD => self.do_add(opcode, false),
            Operation::ADC => self.do_add(opcode, true),
            Operation::ANA => self.do_and(opcode),
            Operation::CMA => self.do_complement_accumulator(opcode),
            Operation::CMC => self.do_complement_carry(opcode),
            Operation::DAA => self.do_decimal_adjust_accumulator(opcode),
            Operation::DCR => self.do_decrement(opcode),
            Operation::INR => self.do_increment(opcode),
            Operation::LDAX => self.do_load_accumulator(opcode),
            Operation::MOV => self.do_move(opcode),
            Operation::NOP => opcode.cycles,
            Operation::RAL => self.do_rotate_left(opcode, true),
            Operation::RAR => self.do_rotate_right(opcode, true),
            Operation::RLC => self.do_rotate_left(opcode, false),
            Operation::RRC => self.do_rotate_right(opcode, false),
            Operation::SBB => self.do_sub(opcode, true),
            Operation::STAX => self.do_store_accumulator(opcode),
            Operation::STC => self.do_set_carry(opcode),
            Operation::SUB => self.do_sub(opcode, false),
            Operation::XRA => self.do_xor(opcode),
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

    fn do_add(&mut self, opcode: &OpCode, with_carry: bool) -> u8 {
        unsafe {
            let to_add = match opcode.code {
                0x80 => self.bc.parts.hi,
                0x81 => self.bc.parts.lo,
                0x82 => self.de.parts.hi,
                0x83 => self.de.parts.lo,
                0x84 => self.hl.parts.hi,
                0x85 => self.hl.parts.lo,
                0x86 => self.read_memory(self.hl.val),
                0x87 => self.af.parts.hi,
                0x88 => self.bc.parts.hi,
                0x89 => self.bc.parts.lo,
                0x8A => self.de.parts.hi,
                0x8B => self.de.parts.lo,
                0x8C => self.hl.parts.hi,
                0x8D => self.hl.parts.lo,
                0x8E => self.read_memory(self.hl.val),
                0x8F => self.af.parts.hi,
                0xC6 => self.get_next_byte(),
                0xCE => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for ADD, ADC, ADI or ACI", opcode.code),
            };

            let carry = if with_carry && self.is_carry_flag_set() { 1 } else { 0 };
            let a_reg = self.af.parts.hi;
            let res = a_reg as usize + to_add as usize + carry;
            let lower_nibble = (a_reg & 0xF) as Word;

            self.update_carry_flag(res > 0xFF);
            self.update_auxiliary_carry_flag(lower_nibble + ((to_add & 0xF) as Word) + carry as Word > 0xF);

            self.af.parts.hi = (res & 0xFF) as Byte;

            self.update_zero_flag(self.af.parts.hi == 0);
            self.update_parity_flag(is_even_parity(&self.af.parts.hi));
            self.update_sign_flag(is_bit_set(&self.af.parts.hi, 7));

            opcode.cycles
        }
    }

    fn do_and(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let to_and = match opcode.code {
                0xA0 => self.bc.parts.hi,
                0xA1 => self.bc.parts.lo,
                0xA2 => self.de.parts.hi,
                0xA3 => self.de.parts.lo,
                0xA4 => self.hl.parts.hi,
                0xA5 => self.hl.parts.lo,
                0xA6 => self.read_memory(self.hl.val),
                0xA7 => self.af.parts.hi,
                0xE6 => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for ANA or ANI", opcode.code),
            };

            self.af.parts.hi &= to_and;

            self.update_zero_flag(self.af.parts.hi == 0);
            self.update_parity_flag(is_even_parity(&self.af.parts.hi));
            self.update_sign_flag(is_bit_set(&self.af.parts.hi, 7));
            self.update_carry_flag(false);
            self.update_auxiliary_carry_flag(false);

            opcode.cycles
        }
    }

    fn do_compare(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let to_cp = match opcode.code {
                0xB8 => self.bc.parts.hi,
                0xB9 => self.bc.parts.lo,
                0xBA => self.de.parts.hi,
                0xBB => self.de.parts.lo,
                0xBC => self.hl.parts.hi,
                0xBD => self.hl.parts.lo,
                0xBE => self.read_memory(self.hl.val),
                0xBF => self.af.parts.hi,
                0xFE => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for CMP or CPI", opcode.code),
            };

            let comp = self.af.parts.hi.wrapping_sub(to_cp);

            self.update_zero_flag(self.af.parts.hi == to_cp);
            self.update_parity_flag(is_even_parity(&comp));
            self.update_sign_flag(is_bit_set(&comp, 7));
            self.update_carry_flag(self.af.parts.hi < to_cp);
            self.update_auxiliary_carry_flag(((self.af.parts.hi as SignedWord) & 0xF) - ((to_cp as SignedWord) & 0xF) < 0);

            opcode.cycles
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

    fn do_load_accumulator(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            match opcode.code {
                0x0A => self.af.parts.hi = self.read_memory(self.bc.val),
                0x1A => self.af.parts.hi = self.read_memory(self.de.val),
                _ => panic!("Unexpected code [{:02X}] encountered for LDAX", opcode.code),
            }
        }

        opcode.cycles
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

    fn do_move(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            match opcode.code {
                0x40 => (),
                0x41 => self.bc.parts.hi = self.bc.parts.lo,
                0x42 => self.bc.parts.hi = self.de.parts.hi,
                0x43 => self.bc.parts.hi = self.de.parts.lo,
                0x44 => self.bc.parts.hi = self.hl.parts.hi,
                0x45 => self.bc.parts.hi = self.hl.parts.lo,
                0x46 => self.bc.parts.hi = self.read_memory(self.hl.val),
                0x47 => self.bc.parts.hi = self.af.parts.hi,
                0x48 => self.bc.parts.lo = self.bc.parts.hi,
                0x49 => (),
                0x4A => self.bc.parts.lo = self.de.parts.hi,
                0x4B => self.bc.parts.lo = self.de.parts.lo,
                0x4C => self.bc.parts.lo = self.hl.parts.hi,
                0x4D => self.bc.parts.lo = self.hl.parts.lo,
                0x4E => self.bc.parts.lo = self.read_memory(self.hl.val),
                0x4F => self.bc.parts.lo = self.af.parts.hi,
                0x50 => self.de.parts.hi = self.bc.parts.hi,
                0x51 => self.de.parts.hi = self.bc.parts.lo,
                0x52 => (),
                0x53 => self.de.parts.hi = self.de.parts.lo,
                0x54 => self.de.parts.hi = self.hl.parts.hi,
                0x55 => self.de.parts.hi = self.hl.parts.lo,
                0x56 => self.de.parts.hi = self.read_memory(self.hl.val),
                0x57 => self.de.parts.hi = self.af.parts.hi,
                0x58 => self.de.parts.lo = self.bc.parts.hi,
                0x59 => self.de.parts.lo = self.bc.parts.lo,
                0x5A => self.de.parts.lo = self.de.parts.hi,
                0x5B => (),
                0x5C => self.de.parts.lo = self.hl.parts.hi,
                0x5D => self.de.parts.lo = self.hl.parts.lo,
                0x5E => self.de.parts.lo = self.read_memory(self.hl.val),
                0x5F => self.de.parts.lo = self.af.parts.hi,
                0x60 => self.hl.parts.hi = self.bc.parts.hi,
                0x61 => self.hl.parts.hi = self.bc.parts.lo,
                0x62 => self.hl.parts.hi = self.de.parts.hi,
                0x63 => self.hl.parts.hi = self.de.parts.lo,
                0x64 => (),
                0x65 => self.hl.parts.hi = self.hl.parts.lo,
                0x66 => self.hl.parts.hi = self.read_memory(self.hl.val),
                0x67 => self.hl.parts.hi = self.af.parts.hi,
                0x68 => self.hl.parts.lo = self.bc.parts.hi,
                0x69 => self.hl.parts.lo = self.bc.parts.lo,
                0x6A => self.hl.parts.lo = self.de.parts.hi,
                0x6B => self.hl.parts.lo = self.de.parts.lo,
                0x6C => self.hl.parts.lo = self.hl.parts.hi,
                0x6D => (),
                0x6E => self.hl.parts.lo = self.read_memory(self.hl.val),
                0x6F => self.hl.parts.lo = self.af.parts.hi,
                0x70 => self.write_memory(self.hl.val, self.bc.parts.hi),
                0x71 => self.write_memory(self.hl.val, self.bc.parts.lo),
                0x72 => self.write_memory(self.hl.val, self.de.parts.hi),
                0x73 => self.write_memory(self.hl.val, self.de.parts.lo),
                0x74 => self.write_memory(self.hl.val, self.hl.parts.hi),
                0x75 => self.write_memory(self.hl.val, self.hl.parts.lo),
                0x77 => self.write_memory(self.hl.val, self.af.parts.hi),
                0x78 => self.af.parts.hi = self.bc.parts.hi,
                0x79 => self.af.parts.hi = self.bc.parts.lo,
                0x7A => self.af.parts.hi = self.de.parts.hi,
                0x7B => self.af.parts.hi = self.de.parts.lo,
                0x7C => self.af.parts.hi = self.hl.parts.hi,
                0x7D => self.af.parts.hi = self.hl.parts.lo,
                0x7E => self.af.parts.hi = self.read_memory(self.hl.val),
                0x7F => (),
                _ => panic!("Unexpected code [{:02X}] encountered for MOV", opcode.code),
            };

            opcode.cycles
        }
    }

    fn do_or(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let to_or = match opcode.code {
                0xB0 => self.bc.parts.hi,
                0xB1 => self.bc.parts.lo,
                0xB2 => self.de.parts.hi,
                0xB3 => self.de.parts.lo,
                0xB4 => self.hl.parts.hi,
                0xB5 => self.hl.parts.lo,
                0xB6 => self.read_memory(self.hl.val),
                0xB7 => self.af.parts.hi,
                0xF6 => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for ORA or ORI", opcode.code),
            };

            self.af.parts.hi |= to_or;

            self.update_zero_flag(self.af.parts.hi == 0);
            self.update_parity_flag(is_even_parity(&self.af.parts.hi));
            self.update_sign_flag(is_bit_set(&self.af.parts.hi, 7));
            self.update_carry_flag(false);
            self.update_auxiliary_carry_flag(false);

            opcode.cycles
        }
    }

    fn do_rotate_left(&mut self, opcode: &OpCode, through_carry: bool) -> u8 {
        unsafe {
            let most_significant_bit = get_bit_val(&self.af.parts.hi, 7);
            let new_least_significant_bit = match through_carry {
                true => get_bit_val(&self.af.parts.lo, CARRY_FLAG as u8),
                false => most_significant_bit,
            };

            self.af.parts.hi = (self.af.parts.hi << 1) | new_least_significant_bit;
            self.update_carry_flag(most_significant_bit == 1);
        }

        opcode.cycles
    }

    fn do_rotate_right(&mut self, opcode: &OpCode, through_carry: bool) -> u8 {
        unsafe {
            let least_significant_bit = get_bit_val(&self.af.parts.hi, 0);
            let new_most_significant_bit = match through_carry {
                true => get_bit_val(&self.af.parts.lo, CARRY_FLAG as u8),
                false => least_significant_bit,
            };

            self.af.parts.hi = (new_most_significant_bit << 7) | (self.af.parts.hi >> 1);
            self.update_carry_flag(least_significant_bit == 1);
        }

        opcode.cycles
    }


    fn do_set_carry(&mut self, opcode: &OpCode) -> u8 {
        self.update_carry_flag(true);
        opcode.cycles
    }

    fn do_store_accumulator(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            match opcode.code {
                0x02 => self.write_memory(self.bc.val, self.af.parts.hi),
                0x12 => self.write_memory(self.de.val, self.af.parts.hi),
                _ => panic!("Unexpected code [{:02X}] encountered for STAX", opcode.code),
            }
        }

        opcode.cycles
    }

    fn do_sub(&mut self, opcode: &OpCode, with_borrow: bool) -> u8 {
        unsafe {
            let to_sub = match opcode.code {
                0x90 => self.bc.parts.hi,
                0x91 => self.bc.parts.lo,
                0x92 => self.de.parts.hi,
                0x93 => self.de.parts.lo,
                0x94 => self.hl.parts.hi,
                0x95 => self.hl.parts.lo,
                0x96 => self.read_memory(self.hl.val),
                0x97 => self.af.parts.hi,
                0x98 => self.bc.parts.hi,
                0x99 => self.bc.parts.lo,
                0x9A => self.de.parts.hi,
                0x9B => self.de.parts.lo,
                0x9C => self.hl.parts.hi,
                0x9D => self.hl.parts.lo,
                0x9E => self.read_memory(self.hl.val),
                0x9F => self.af.parts.hi,
                0xD6 => self.get_next_byte(),
                0xDE => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for SUB, SUB, SUI or SBI", opcode.code),
            };

            let carry = if with_borrow && self.is_carry_flag_set() {1} else {0};
            let a_reg = self.af.parts.hi;
            let res = a_reg.wrapping_sub(to_sub).wrapping_sub(carry);
            let lower_nibble = (a_reg & 0xF) as Word;

            self.update_zero_flag(res == 0);
            self.update_parity_flag(is_even_parity(&res));
            self.update_carry_flag((a_reg as Word) < (to_sub as Word) + (carry as Word));
            self.update_auxiliary_carry_flag((a_reg & 0xF) < (to_sub & 0xF) + (carry as Byte));
            self.update_sign_flag(is_bit_set(&res, 7));

            self.af.parts.hi = res;

            opcode.cycles
        }
    }

    fn do_xor(&mut self, opcode: &OpCode) -> u8 {
        unsafe {
            let to_xor = match opcode.code {
                0xA8 => self.bc.parts.hi,
                0xA9 => self.bc.parts.lo,
                0xAA => self.de.parts.hi,
                0xAB => self.de.parts.lo,
                0xAC => self.hl.parts.hi,
                0xAD => self.hl.parts.lo,
                0xAE => self.read_memory(self.hl.val),
                0xAF => self.af.parts.hi,
                0xEE => self.get_next_byte(),
                _ => panic!("Unexpected code [{:02X}] encountered for XRA or XRI", opcode.code),
            };

            self.af.parts.hi ^= to_xor;

            self.update_zero_flag(self.af.parts.hi == 0);
            self.update_parity_flag(is_even_parity(&self.af.parts.hi));
            self.update_sign_flag(is_bit_set(&self.af.parts.hi, 7));
            self.update_carry_flag(false);
            self.update_auxiliary_carry_flag(false);

            opcode.cycles
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_add() {
        let mut cpu = Cpu::new(Bus::new());

        let code = 0x80;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0xD0;
        cpu.bc.parts.hi = 0x01;
        cpu.do_add(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0xD1);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        cpu.bc.parts.hi = 0x2F;
        cpu.do_add(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0);
            assert_eq!(cpu.is_zero_flag_set(), true);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_carry_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        let code = 0x88;
        let opcode = OPCODE_MAP.get(&code).unwrap();
        cpu.bc.parts.hi = 0x05;
        cpu.do_add(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0x06);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

    #[test]
    fn test_do_and() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0xA0;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b11110000;
        cpu.bc.parts.hi = 0b11001100;
        cpu.do_and(&opcode);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b11000000);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

    #[test]
    fn test_do_compare() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0xB8;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0xD0;
        cpu.bc.parts.hi = 0x01;
        cpu.do_compare(&opcode);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0xD0);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

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

    #[test]
    fn test_do_load_accumulator() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x1A;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.de.val = 0x1234;

        unsafe {
            cpu.write_memory(cpu.de.val, 0xD0);
            cpu.do_load_accumulator(&opcode);
            assert_eq!(cpu.af.parts.hi, 0xD0);
        }
    }

    #[test]
    fn test_do_move() {
        let mut cpu = Cpu::new(Bus::new());

        let code = 0x41;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.bc.parts.lo = 0xD0;
        cpu.do_move(&opcode);

        unsafe { assert_eq!(cpu.bc.parts.hi, 0xD0); }

        let code = 0x53;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.de.parts.lo = 0x45;
        cpu.do_move(&opcode);

        unsafe { assert_eq!(cpu.de.parts.hi, 0x45); }

        let code = 0x4E;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.hl.val = 0x1234;
        unsafe { cpu.write_memory(cpu.hl.val, 0x23); }
        cpu.do_move(&opcode);

        unsafe { assert_eq!(cpu.bc.parts.lo, 0x23); }
    }

    #[test]
    fn test_do_rotate_left() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x07;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b10101010;
        cpu.do_rotate_left(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b01010101);
            assert_eq!(cpu.is_carry_flag_set(), true);
        }

        cpu.do_rotate_left(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b10101010);
            assert_eq!(cpu.is_carry_flag_set(), false);
        }

        let code = 0x17;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.do_rotate_left(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b01010100);
            assert_eq!(cpu.is_carry_flag_set(), true);
        }

        cpu.do_rotate_left(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b10101001);
            assert_eq!(cpu.is_carry_flag_set(), false);
        }
    }

    #[test]
    fn test_do_rotate_right() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x0F;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b10101010;
        cpu.do_rotate_right(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b01010101);
            assert_eq!(cpu.is_carry_flag_set(), false);
        }

        cpu.do_rotate_right(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b10101010);
            assert_eq!(cpu.is_carry_flag_set(), true);
        }

        let code = 0x1F;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.do_rotate_right(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b11010101);
            assert_eq!(cpu.is_carry_flag_set(), false);
        }

        cpu.do_rotate_right(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b01101010);
            assert_eq!(cpu.is_carry_flag_set(), true);
        }
    }

    #[test]
    fn test_do_store_accumulator() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0x02;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.bc.val = 0x1234;

        unsafe {
            cpu.af.parts.hi = 0xD0;
            cpu.do_store_accumulator(&opcode);
            assert_eq!(cpu.read_memory(cpu.bc.val), 0xD0);
        }
    }

    #[test]
    fn test_do_sub() {
        let mut cpu = Cpu::new(Bus::new());

        let code = 0x90;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0xD0;
        cpu.bc.parts.hi = 0x01;
        cpu.do_sub(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0xCF);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), true);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        cpu.bc.parts.hi = 0xD0;
        cpu.do_sub(&opcode, false);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0xFF);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), true);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }

        let code = 0x98;
        let opcode = OPCODE_MAP.get(&code).unwrap();
        cpu.bc.parts.hi = 0x05;
        cpu.do_sub(&opcode, true);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0xF9);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

    #[test]
    fn test_do_xor() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0xA8;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b11110000;
        cpu.bc.parts.hi = 0b11001100;
        cpu.do_xor(&opcode);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b00111100);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), false);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }

    #[test]
    fn test_do_or() {
        let mut cpu = Cpu::new(Bus::new());
        let code = 0xB0;
        let opcode = OPCODE_MAP.get(&code).unwrap();

        cpu.af.parts.hi = 0b11110000;
        cpu.bc.parts.hi = 0b11001100;
        cpu.do_or(&opcode);

        unsafe {
            assert_eq!(cpu.af.parts.hi, 0b11111100);
            assert_eq!(cpu.is_zero_flag_set(), false);
            assert_eq!(cpu.is_sign_flag_set(), true);
            assert_eq!(cpu.is_carry_flag_set(), false);
            assert_eq!(cpu.is_auxiliary_carry_flag_set(), false);
            assert_eq!(cpu.is_parity_flag_set(), true);
        }
    }
}