use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use crate::constants::*;
use crate::utils::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operation {
    CMA,
    CMC,
    DAA,
    DCR,
    INR,
    NOP,
    STC,
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
        OpCode::new(0x04, String::from("INR B"), Operation::INR, 1, 5, None),
        OpCode::new(0x05, String::from("DCR B"), Operation::DCR, 1, 5, None),
        OpCode::new(0x0C, String::from("INR C"), Operation::INR, 1, 5, None),
        OpCode::new(0x0D, String::from("DCR C"), Operation::DCR, 1, 5, None),

        OpCode::new(0x14, String::from("INR D"), Operation::INR, 1, 5, None),
        OpCode::new(0x15, String::from("DCR D"), Operation::DCR, 1, 5, None),
        OpCode::new(0x1C, String::from("INR E"), Operation::INR, 1, 5, None),
        OpCode::new(0x1D, String::from("DCR E"), Operation::DCR, 1, 5, None),

        OpCode::new(0x24, String::from("INR H"), Operation::INR, 1, 5, None),
        OpCode::new(0x25, String::from("DCR H"), Operation::DCR, 1, 5, None),
        OpCode::new(0x27, String::from("DAA"), Operation::DAA, 1, 4, None),
        OpCode::new(0x2C, String::from("INR L"), Operation::INR, 1, 5, None),
        OpCode::new(0x2D, String::from("DCR L"), Operation::DCR, 1, 5, None),
        OpCode::new(0x2F, String::from("CMA"), Operation::CMA, 1, 4, None),

        OpCode::new(0x34, String::from("INR M"), Operation::INR, 1, 10, None),
        OpCode::new(0x35, String::from("DCR M"), Operation::DCR, 1, 10, None),
        OpCode::new(0x37, String::from("STC"), Operation::STC, 1, 4, None),
        OpCode::new(0x3C, String::from("INR A"), Operation::INR, 1, 5, None),
        OpCode::new(0x3D, String::from("DCR A"), Operation::DCR, 1, 5, None),
        OpCode::new(0x3F, String::from("CMC"), Operation::CMC, 1, 4, None),
    ];

    pub static ref OPCODE_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPU_OP_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}