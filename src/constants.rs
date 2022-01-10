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
