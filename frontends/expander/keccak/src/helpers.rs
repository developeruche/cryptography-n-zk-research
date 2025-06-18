use crate::constants::rc;
use expander_compiler::frontend::{Config, RootAPI, Variable};

pub(crate) fn keccak_f<C: Config, B: RootAPI<C>>(
    api: &mut B,
    mut a: Vec<Vec<Variable>>,
) -> Vec<Vec<Variable>> {
    let mut b = vec![vec![api.constant(0); 64]; 25];
    let mut c = vec![vec![api.constant(0); 64]; 5];
    let mut d = vec![vec![api.constant(0); 64]; 5];
    let mut da = vec![vec![api.constant(0); 64]; 5];
    let rc = rc();

    for i in 0..24 {
        for j in 0..5 {
            let t1 = xor(api, a[j * 5 + 1].clone(), a[j * 5 + 2].clone());
            let t2 = xor(api, a[j * 5 + 3].clone(), a[j * 5 + 4].clone());
            c[j] = xor(api, t1, t2);
        }

        for j in 0..5 {
            d[j] = xor(
                api,
                c[(j + 4) % 5].clone(),
                rotate_left::<C>(&c[(j + 1) % 5], 1),
            );
            da[j] = xor(
                api,
                a[((j + 4) % 5) * 5].clone(),
                rotate_left::<C>(&a[((j + 1) % 5) * 5], 1),
            );
        }

        for j in 0..25 {
            let tmp = xor(api, da[j / 5].clone(), a[j].clone());
            a[j] = xor(api, tmp, d[j / 5].clone());
        }

        /*Rho and pi steps*/
        b[0] = a[0].clone();

        b[8] = rotate_left::<C>(&a[1], 36);
        b[11] = rotate_left::<C>(&a[2], 3);
        b[19] = rotate_left::<C>(&a[3], 41);
        b[22] = rotate_left::<C>(&a[4], 18);

        b[2] = rotate_left::<C>(&a[5], 1);
        b[5] = rotate_left::<C>(&a[6], 44);
        b[13] = rotate_left::<C>(&a[7], 10);
        b[16] = rotate_left::<C>(&a[8], 45);
        b[24] = rotate_left::<C>(&a[9], 2);

        b[4] = rotate_left::<C>(&a[10], 62);
        b[7] = rotate_left::<C>(&a[11], 6);
        b[10] = rotate_left::<C>(&a[12], 43);
        b[18] = rotate_left::<C>(&a[13], 15);
        b[21] = rotate_left::<C>(&a[14], 61);

        b[1] = rotate_left::<C>(&a[15], 28);
        b[9] = rotate_left::<C>(&a[16], 55);
        b[12] = rotate_left::<C>(&a[17], 25);
        b[15] = rotate_left::<C>(&a[18], 21);
        b[23] = rotate_left::<C>(&a[19], 56);

        b[3] = rotate_left::<C>(&a[20], 27);
        b[6] = rotate_left::<C>(&a[21], 20);
        b[14] = rotate_left::<C>(&a[22], 39);
        b[17] = rotate_left::<C>(&a[23], 8);
        b[20] = rotate_left::<C>(&a[24], 14);

        /*Xi state*/

        for j in 0..25 {
            let t = not(api, b[(j + 5) % 25].clone());
            let t = and(api, t, b[(j + 10) % 25].clone());
            a[j] = xor(api, b[j].clone(), t);
        }

        /*Last step*/

        for j in 0..64 {
            if rc[i] >> j & 1 == 1 {
                a[0][j] = api.sub(1, a[0][j]);
            }
        }
    }

    a
}

pub(crate) fn xor_in<C: Config, B: RootAPI<C>>(
    api: &mut B,
    mut s: Vec<Vec<Variable>>,
    buf: Vec<Vec<Variable>>,
) -> Vec<Vec<Variable>> {
    for y in 0..5 {
        for x in 0..5 {
            if x + 5 * y < buf.len() {
                s[5 * x + y] = xor(api, s[5 * x + y].clone(), buf[x + 5 * y].clone())
            }
        }
    }
    s
}

pub(crate) fn xor<C: Config, B: RootAPI<C>>(
    api: &mut B,
    a: Vec<Variable>,
    b: Vec<Variable>,
) -> Vec<Variable> {
    let nbits = a.len();
    let mut bits_res = vec![api.constant(0); nbits];
    for i in 0..nbits {
        bits_res[i] = api.add(a[i].clone(), b[i].clone());
    }
    bits_res
}

pub(crate) fn and<C: Config, B: RootAPI<C>>(
    api: &mut B,
    a: Vec<Variable>,
    b: Vec<Variable>,
) -> Vec<Variable> {
    let nbits = a.len();
    let mut bits_res = vec![api.constant(0); nbits];
    for i in 0..nbits {
        bits_res[i] = api.mul(a[i].clone(), b[i].clone());
    }
    bits_res
}

pub(crate) fn not<C: Config, B: RootAPI<C>>(api: &mut B, a: Vec<Variable>) -> Vec<Variable> {
    let mut bits_res = vec![api.constant(0); a.len()];
    for i in 0..a.len() {
        bits_res[i] = api.sub(1, a[i].clone());
    }
    bits_res
}

pub(crate) fn rotate_left<C: Config>(bits: &Vec<Variable>, k: usize) -> Vec<Variable> {
    let n = bits.len();
    let s = k & (n - 1);
    let mut new_bits = bits[(n - s) as usize..].to_vec();
    new_bits.append(&mut bits[0..(n - s) as usize].to_vec());
    new_bits
}

pub(crate) fn copy_out_unaligned(
    s: Vec<Vec<Variable>>,
    rate: usize,
    output_len: usize,
) -> Vec<Variable> {
    let mut out = vec![];
    let w = 8;
    let mut b = 0;
    while b < output_len {
        for y in 0..5 {
            for x in 0..5 {
                if x + 5 * y < rate / w && b < output_len {
                    out.append(&mut s[5 * x + y].clone());
                    b += 8;
                }
            }
        }
    }
    out
}
