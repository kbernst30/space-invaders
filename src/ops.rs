use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use crate::constants::*;
use crate::utils::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    ADD,
    ADC,
    ANA,
    CMA,
    CMC,
    CMP,
    DAA,
    DAD,
    DCR,
    DCX,
    INR,
    INX,
    LDAX,
    MOV,
    NOP,
    ORA,
    POP,
    PUSH,
    RAL,
    RAR,
    RLC,
    RRC,
    SBB,
    SPHL,
    STAX,
    STC,
    SUB,
    XCHG,
    XRA,
    XTHL,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct OpCode {
    pub code: Byte,
    pub mnemonic: String,
    pub operation: Operation,
    pub len: u8,
    pub cycles: u8,
    pub alt_cycles: Option<u8>,
}

impl OpCode {
    fn new(code: Byte, mnemonic: String, operation: Operation, len: u8, cycles: u8, alt_cycles: Option<u8>) -> OpCode {
        OpCode {
            code: code,
            mnemonic: mnemonic,
            operation: operation,
            len: len,
            cycles: cycles,
            alt_cycles: alt_cycles
        }
    }
}

lazy_static! {
    pub static ref CPU_OP_CODES: Vec<OpCode> = vec![
        OpCode::new(0x00, String::from("NOP"), Operation::NOP, 1, 4, None),
        OpCode::new(0x02, String::from("STAX B"), Operation::STAX, 1, 7, None),
        OpCode::new(0x03, String::from("INX B"), Operation::INX, 1, 5, None),
        OpCode::new(0x04, String::from("INR B"), Operation::INR, 1, 5, None),
        OpCode::new(0x05, String::from("DCR B"), Operation::DCR, 1, 5, None),
        OpCode::new(0x07, String::from("RLC"), Operation::RLC, 1, 4, None),
        OpCode::new(0x09, String::from("DAD B"), Operation::DAD, 1, 10, None),
        OpCode::new(0x0A, String::from("LDAX B"), Operation::LDAX, 1, 7, None),
        OpCode::new(0x0B, String::from("DCX B"), Operation::DCX, 1, 5, None),
        OpCode::new(0x0C, String::from("INR C"), Operation::INR, 1, 5, None),
        OpCode::new(0x0D, String::from("DCR C"), Operation::DCR, 1, 5, None),
        OpCode::new(0x0F, String::from("RRC"), Operation::RRC, 1, 4, None),

        OpCode::new(0x12, String::from("STAX D"), Operation::STAX, 1, 7, None),
        OpCode::new(0x13, String::from("INX D"), Operation::INX, 1, 5, None),
        OpCode::new(0x14, String::from("INR D"), Operation::INR, 1, 5, None),
        OpCode::new(0x15, String::from("DCR D"), Operation::DCR, 1, 5, None),
        OpCode::new(0x17, String::from("RAL"), Operation::RAL, 1, 4, None),
        OpCode::new(0x19, String::from("DAD D"), Operation::DAD, 1, 10, None),
        OpCode::new(0x1A, String::from("LDAX B"), Operation::LDAX, 1, 7, None),
        OpCode::new(0x1B, String::from("DCX D"), Operation::DCX, 1, 5, None),
        OpCode::new(0x1C, String::from("INR E"), Operation::INR, 1, 5, None),
        OpCode::new(0x1D, String::from("DCR E"), Operation::DCR, 1, 5, None),
        OpCode::new(0x1F, String::from("RAR"), Operation::RAR, 1, 4, None),

        OpCode::new(0x23, String::from("INX H"), Operation::INX, 1, 5, None),
        OpCode::new(0x24, String::from("INR H"), Operation::INR, 1, 5, None),
        OpCode::new(0x25, String::from("DCR H"), Operation::DCR, 1, 5, None),
        OpCode::new(0x27, String::from("DAA"), Operation::DAA, 1, 4, None),
        OpCode::new(0x29, String::from("DAD H"), Operation::DAD, 1, 10, None),
        OpCode::new(0x2B, String::from("DCX H"), Operation::DCX, 1, 5, None),
        OpCode::new(0x2C, String::from("INR L"), Operation::INR, 1, 5, None),
        OpCode::new(0x2D, String::from("DCR L"), Operation::DCR, 1, 5, None),
        OpCode::new(0x2F, String::from("CMA"), Operation::CMA, 1, 4, None),

        OpCode::new(0x33, String::from("INX SP"), Operation::INX, 1, 5, None),
        OpCode::new(0x34, String::from("INR M"), Operation::INR, 1, 10, None),
        OpCode::new(0x35, String::from("DCR M"), Operation::DCR, 1, 10, None),
        OpCode::new(0x37, String::from("STC"), Operation::STC, 1, 4, None),
        OpCode::new(0x39, String::from("DAD SP"), Operation::DAD, 1, 10, None),
        OpCode::new(0x3B, String::from("DCX SP"), Operation::DCX, 1, 5, None),
        OpCode::new(0x3C, String::from("INR A"), Operation::INR, 1, 5, None),
        OpCode::new(0x3D, String::from("DCR A"), Operation::DCR, 1, 5, None),
        OpCode::new(0x3F, String::from("CMC"), Operation::CMC, 1, 4, None),

        OpCode::new(0x40, String::from("MOV B,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x41, String::from("MOV B,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x42, String::from("MOV B,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x43, String::from("MOV B,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x44, String::from("MOV B,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x45, String::from("MOV B,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x46, String::from("MOV B,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x47, String::from("MOV B,A"), Operation::MOV, 1, 5, None),
        OpCode::new(0x48, String::from("MOV C,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x49, String::from("MOV C,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x4A, String::from("MOV C,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x4B, String::from("MOV C,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x4C, String::from("MOV C,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x4D, String::from("MOV C,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x4E, String::from("MOV C,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x4F, String::from("MOV C,A"), Operation::MOV, 1, 5, None),

        OpCode::new(0x50, String::from("MOV D,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x51, String::from("MOV D,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x52, String::from("MOV D,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x53, String::from("MOV D,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x54, String::from("MOV D,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x55, String::from("MOV D,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x56, String::from("MOV D,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x57, String::from("MOV D,A"), Operation::MOV, 1, 5, None),
        OpCode::new(0x58, String::from("MOV E,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x59, String::from("MOV E,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x5A, String::from("MOV E,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x5B, String::from("MOV E,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x5C, String::from("MOV E,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x5D, String::from("MOV E,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x5E, String::from("MOV E,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x5F, String::from("MOV E,A"), Operation::MOV, 1, 5, None),

        OpCode::new(0x60, String::from("MOV H,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x61, String::from("MOV H,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x62, String::from("MOV H,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x63, String::from("MOV H,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x64, String::from("MOV H,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x65, String::from("MOV H,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x66, String::from("MOV H,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x67, String::from("MOV H,A"), Operation::MOV, 1, 5, None),
        OpCode::new(0x68, String::from("MOV L,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x69, String::from("MOV L,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x6A, String::from("MOV L,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x6B, String::from("MOV L,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x6C, String::from("MOV L,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x6D, String::from("MOV L,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x6E, String::from("MOV L,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x6F, String::from("MOV L,A"), Operation::MOV, 1, 5, None),

        OpCode::new(0x70, String::from("MOV M,B"), Operation::MOV, 1, 7, None),
        OpCode::new(0x71, String::from("MOV M,C"), Operation::MOV, 1, 7, None),
        OpCode::new(0x72, String::from("MOV M,D"), Operation::MOV, 1, 7, None),
        OpCode::new(0x73, String::from("MOV M,E"), Operation::MOV, 1, 7, None),
        OpCode::new(0x74, String::from("MOV M,H"), Operation::MOV, 1, 7, None),
        OpCode::new(0x75, String::from("MOV M,L"), Operation::MOV, 1, 7, None),
        // OpCode::new(0x76, String::from("MOV B,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x77, String::from("MOV M,A"), Operation::MOV, 1, 7, None),
        OpCode::new(0x78, String::from("MOV A,B"), Operation::MOV, 1, 5, None),
        OpCode::new(0x79, String::from("MOV A,C"), Operation::MOV, 1, 5, None),
        OpCode::new(0x7A, String::from("MOV A,D"), Operation::MOV, 1, 5, None),
        OpCode::new(0x7B, String::from("MOV A,E"), Operation::MOV, 1, 5, None),
        OpCode::new(0x7C, String::from("MOV A,H"), Operation::MOV, 1, 5, None),
        OpCode::new(0x7D, String::from("MOV A,L"), Operation::MOV, 1, 5, None),
        OpCode::new(0x7E, String::from("MOV A,M"), Operation::MOV, 1, 7, None),
        OpCode::new(0x7F, String::from("MOV A,A"), Operation::MOV, 1, 5, None),

        OpCode::new(0x80, String::from("ADD B"), Operation::ADD, 1, 4, None),
        OpCode::new(0x81, String::from("ADD C"), Operation::ADD, 1, 4, None),
        OpCode::new(0x82, String::from("ADD D"), Operation::ADD, 1, 4, None),
        OpCode::new(0x83, String::from("ADD E"), Operation::ADD, 1, 4, None),
        OpCode::new(0x84, String::from("ADD H"), Operation::ADD, 1, 4, None),
        OpCode::new(0x85, String::from("ADD L"), Operation::ADD, 1, 4, None),
        OpCode::new(0x86, String::from("ADD M"), Operation::ADD, 1, 7, None),
        OpCode::new(0x87, String::from("ADD A"), Operation::ADD, 1, 4, None),
        OpCode::new(0x88, String::from("ADC B"), Operation::ADC, 1, 4, None),
        OpCode::new(0x89, String::from("ADC C"), Operation::ADC, 1, 4, None),
        OpCode::new(0x8A, String::from("ADC D"), Operation::ADC, 1, 4, None),
        OpCode::new(0x8B, String::from("ADC E"), Operation::ADC, 1, 4, None),
        OpCode::new(0x8C, String::from("ADC H"), Operation::ADC, 1, 4, None),
        OpCode::new(0x8D, String::from("ADC L"), Operation::ADC, 1, 4, None),
        OpCode::new(0x8E, String::from("ADC M"), Operation::ADC, 1, 7, None),
        OpCode::new(0x8F, String::from("ADC A"), Operation::ADC, 1, 4, None),

        OpCode::new(0x90, String::from("SUB B"), Operation::SUB, 1, 4, None),
        OpCode::new(0x91, String::from("SUB C"), Operation::SUB, 1, 4, None),
        OpCode::new(0x92, String::from("SUB D"), Operation::SUB, 1, 4, None),
        OpCode::new(0x93, String::from("SUB E"), Operation::SUB, 1, 4, None),
        OpCode::new(0x94, String::from("SUB H"), Operation::SUB, 1, 4, None),
        OpCode::new(0x95, String::from("SUB L"), Operation::SUB, 1, 4, None),
        OpCode::new(0x96, String::from("SUB M"), Operation::SUB, 1, 7, None),
        OpCode::new(0x97, String::from("SUB A"), Operation::SUB, 1, 4, None),
        OpCode::new(0x98, String::from("SBB B"), Operation::SBB, 1, 4, None),
        OpCode::new(0x99, String::from("SBB C"), Operation::SBB, 1, 4, None),
        OpCode::new(0x9A, String::from("SBB D"), Operation::SBB, 1, 4, None),
        OpCode::new(0x9B, String::from("SBB E"), Operation::SBB, 1, 4, None),
        OpCode::new(0x9C, String::from("SBB H"), Operation::SBB, 1, 4, None),
        OpCode::new(0x9D, String::from("SBB L"), Operation::SBB, 1, 4, None),
        OpCode::new(0x9E, String::from("SBB M"), Operation::SBB, 1, 7, None),
        OpCode::new(0x9F, String::from("SBB A"), Operation::SBB, 1, 4, None),

        OpCode::new(0xA0, String::from("ANA B"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA1, String::from("ANA C"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA2, String::from("ANA D"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA3, String::from("ANA E"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA4, String::from("ANA H"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA5, String::from("ANA L"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA6, String::from("ANA M"), Operation::ANA, 1, 7, None),
        OpCode::new(0xA7, String::from("ANA A"), Operation::ANA, 1, 4, None),
        OpCode::new(0xA8, String::from("XRA B"), Operation::XRA, 1, 4, None),
        OpCode::new(0xA9, String::from("XRA C"), Operation::XRA, 1, 4, None),
        OpCode::new(0xAA, String::from("XRA D"), Operation::XRA, 1, 4, None),
        OpCode::new(0xAB, String::from("XRA E"), Operation::XRA, 1, 4, None),
        OpCode::new(0xAC, String::from("XRA H"), Operation::XRA, 1, 4, None),
        OpCode::new(0xAD, String::from("XRA L"), Operation::XRA, 1, 4, None),
        OpCode::new(0xAE, String::from("XRA M"), Operation::XRA, 1, 7, None),
        OpCode::new(0xAF, String::from("XRA A"), Operation::XRA, 1, 4, None),

        OpCode::new(0xB0, String::from("ORA B"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB1, String::from("ORA C"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB2, String::from("ORA D"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB3, String::from("ORA E"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB4, String::from("ORA H"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB5, String::from("ORA L"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB6, String::from("ORA M"), Operation::ORA, 1, 7, None),
        OpCode::new(0xB7, String::from("ORA A"), Operation::ORA, 1, 4, None),
        OpCode::new(0xB8, String::from("CMP B"), Operation::CMP, 1, 4, None),
        OpCode::new(0xB9, String::from("CMP C"), Operation::CMP, 1, 4, None),
        OpCode::new(0xBA, String::from("CMP D"), Operation::CMP, 1, 4, None),
        OpCode::new(0xBB, String::from("CMP E"), Operation::CMP, 1, 4, None),
        OpCode::new(0xBC, String::from("CMP H"), Operation::CMP, 1, 4, None),
        OpCode::new(0xBD, String::from("CMP L"), Operation::CMP, 1, 4, None),
        OpCode::new(0xBE, String::from("CMP M"), Operation::CMP, 1, 7, None),
        OpCode::new(0xBF, String::from("CMP A"), Operation::CMP, 1, 4, None),

        OpCode::new(0xC1, String::from("POP B"), Operation::POP, 1, 10, None),
        OpCode::new(0xC5, String::from("PUSH B"), Operation::PUSH, 1, 11, None),

        OpCode::new(0xD1, String::from("POP D"), Operation::POP, 1, 10, None),
        OpCode::new(0xD5, String::from("PUSH D"), Operation::PUSH, 1, 11, None),

        OpCode::new(0xE1, String::from("POP H"), Operation::POP, 1, 10, None),
        OpCode::new(0xE3, String::from("XTHL"), Operation::XTHL, 1, 18, None),
        OpCode::new(0xE5, String::from("PUSH H"), Operation::PUSH, 1, 11, None),
        OpCode::new(0xEB, String::from("XCHG"), Operation::XCHG, 1, 5, None),

        OpCode::new(0xF1, String::from("POP PSW"), Operation::POP, 1, 10, None),
        OpCode::new(0xF5, String::from("PUSH PSW"), Operation::PUSH, 1, 11, None),
        OpCode::new(0xF9, String::from("SPHL"), Operation::SPHL, 1, 5, None),

    ];

    pub static ref OPCODE_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OP_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}