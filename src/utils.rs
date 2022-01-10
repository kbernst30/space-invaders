pub fn is_bit_set(data: &Byte, position: usize) -> bool {
    // Return true if bit at position is
    // set in data, false otherwise
    (data & (1 << position)) > 0
}

pub fn set_bit(data: &mut Byte, position: usize) {
    let setter = 1 << position;
    *data |= setter;
}

pub fn reset_bit(data: &mut Byte, position: usize) {
    let setter = !(1 << position); // Bit wise negate to get a 0 in the appropriate pos
    *data &= setter;
}

pub fn get_bit_val(data: &Byte, position: u8) -> u8 {
    match (data & (1 << position)) > 0 {
        true => 1,
        false => 0
    }
}