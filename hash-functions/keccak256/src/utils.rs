//! Houses utility functions used in Keccak256 hashing algorithm.

use crate::{
    constants::{ROTATION_CONSTANTS, ROUND_CONSTANTS},
    keccak::KeccakState,
};

pub fn create_masks() -> Vec<u64> {
    (0..65)
        .map(|i| {
            if i == 0 {
                0
            } else if i == 64 {
                u64::MAX
            } else {
                (1u64 << i) - 1
            }
        })
        .collect()
}

pub fn bits2bytes(x: usize) -> usize {
    (x + 7) / 8
}

pub fn rol(value: u64, left: usize, bits: usize) -> u64 {
    let mask = if bits < 64 { (1u64 << bits) - 1 } else { !0u64 };
    let top = value >> (bits - left);
    let bot = (value & ((1u64 << (bits - left)) - 1)) << left;
    (bot | top) & mask
}

pub fn ror(value: u64, right: usize, bits: usize) -> u64 {
    let mask = if bits < 64 { (1u64 << bits) - 1 } else { !0u64 };
    let top = value >> right;
    let bot = (value & ((1u64 << right) - 1)) << (bits - right);
    (bot | top) & mask
}

pub fn multirate_padding(used_bytes: usize, align_bytes: usize) -> Vec<u8> {
    let mut padlen = align_bytes - used_bytes;
    if padlen == 0 {
        padlen = align_bytes;
    }

    if padlen == 1 {
        vec![0x81]
    } else {
        let mut result = vec![0x01];
        result.extend(vec![0x00; padlen - 2]);
        result.push(0x80);
        result
    }
}

// The main keccak_round function
fn keccak_round(a: &mut Vec<Vec<u64>>, rc: u64, lanew: usize) {
    let w = 5;
    let h = 5;

    // theta
    let mut c = vec![0u64; w];
    for x in 0..w {
        c[x] = a[x].iter().fold(0, |acc, &val| acc ^ val);
    }

    let mut d = vec![0u64; w];
    for x in 0..w {
        d[x] = c[(w + x - 1) % w] ^ rol(c[(x + 1) % w], 1, lanew);
        for y in 0..h {
            a[x][y] ^= d[x];
        }
    }

    // rho and pi
    let mut b = vec![vec![0u64; h]; w];
    for x in 0..w {
        for y in 0..h {
            b[y % w][(2 * x + 3 * y) % h] = rol(a[x][y], ROTATION_CONSTANTS[y][x] as usize, lanew);
        }
    }

    // chi
    for x in 0..w {
        for y in 0..h {
            a[x][y] = b[x][y] ^ ((!b[(x + 1) % w][y]) & b[(x + 2) % w][y]);
        }
    }

    // iota
    a[0][0] ^= rc;
}

// The main keccak_f function
pub fn keccak_f(state: &mut KeccakState) {
    let nr = 12 + 2 * (state.lanew as f64).log2() as usize;

    for ir in 0..nr {
        keccak_round(&mut state.s, ROUND_CONSTANTS[ir], state.lanew);
    }
}
