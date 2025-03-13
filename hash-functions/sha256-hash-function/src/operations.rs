/// This function is used to perform the XOR operation
pub fn xor(a: u32, b: u32) -> u32 {
    a ^ b
}

/// This is for doing a bit shift right
pub fn right_shift(x: u32, round: u32) -> u32 {
    x >> round
}

/// This is for doing a rotate right
pub fn rotate_right(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}
