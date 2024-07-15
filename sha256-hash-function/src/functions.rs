use std::ops::Shr;

pub fn sigma_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ x.shr(3)
}

pub fn sigma_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ x.shr(10)
}

pub fn prime_sigma_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

pub fn prime_sigma_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

pub fn choice(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

pub fn compute_message_shedule_extension(wi_2: u32, wi_7: u32, wi_15: u32, wi_16: u32) -> u32 {
    sigma_1(wi_2)
        .wrapping_add(wi_7)
        .wrapping_add(sigma_1(wi_15))
        .wrapping_add(wi_16)
}

pub fn convert_to_u32(padded_vec: Vec<u8>) -> Vec<u32> {
    if padded_vec.len() != 112 {
        panic!("Input vector must be of length 112");
    }

    let mut u32_vec = Vec::new();
    for i in (0..padded_vec.len()).step_by(4) {
        // Safe since the check ensures length is a multiple of 4
        let u32_element = unsafe {
            // Convert a slice of 4 bytes to u32 (assuming native endianness)
            *(padded_vec.as_ptr().add(i) as *const u32)
        };
        u32_vec.push(u32_element);
    }
    u32_vec
}

pub fn split_u64_to_u32(value: u64) -> Vec<u32> {
    let upper_32 = (value >> 32) as u32; // Right shift by 32 to get upper bits and cast to u32
    let lower_32 = (value & 0xffffffff) as u32; // Bitwise AND with all ones to get lower bits and cast to u32
    vec![upper_32, lower_32]
}
