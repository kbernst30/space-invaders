pub type Byte = u8;
pub type SignedByte = i8;
pub type Word = u16;
pub type SignedWord = i16;

// The resolution is 256x224 but the monitor is rotated in the cabinet 90 degrees counter-clockwise,
// So set variables for Display for SDL that are already rotates
pub const DISPLAY_WIDTH: u32 = 224;
pub const DISPLAY_HEIGHT: u32 = 256;
pub const DISPLAY_FACTOR: u32 = 2;

// We can address from 0 - 65535 in memory (i.e. 0x0000 - 0xFFFF)
pub const MEMORY_SIZE: Word = 65536;

// Flags
// The following bits are used with register A as a flags Register. The following bits
// represent the following flags:
// 7    S   Sign flag
// 6	Z	Zero flag
// 5    0   Unused
// 4    A   Auxiliary Carry Flag (carry from bit 3)
// 3    0   Unused
// 2    P   Parity Flag
// 1    1   Unused
// 0    C   Carry Flag
pub const SIGN_FLAG: usize = 7;
pub const ZERO_FLAG: usize = 6;
pub const AUXILIARY_CARRY_FLAG: usize = 4;
pub const PARITY_FLAG: usize = 2;
pub const CARRY_FLAG: usize = 0;