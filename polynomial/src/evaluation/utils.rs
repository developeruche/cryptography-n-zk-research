use ark_ff::PrimeField;

pub fn split_odd_even<F: PrimeField>(arr: &Vec<F>) -> (Vec<F>, Vec<F>) {
    let even: Vec<F> = arr.iter().step_by(2).cloned().collect();
    let odd: Vec<F> = arr.iter().skip(1).step_by(2).cloned().collect();

    (even, odd)
}

/// this algorithm was for from dusk-plonk implemenation
pub fn serial_fft<F: PrimeField>(list: &mut Vec<F>, w: F, size_log: u32) {
    let n = list.len() as u32;
    // this is also a check ensure that the size of the list is a power of 2
    assert_eq!(n, 1 << size_log);

    for k in 0..n {
        let rk = bitreverse(k, size_log);
        if k < rk {
            list.swap(rk as usize, k as usize);
        }
    }

    let mut m = 1;
    for _ in 0..size_log {
        let w_m = w.pow(&[(n / (2 * m)) as u64, 0, 0, 0]);

        let mut k = 0;
        while k < n {
            let mut w = F::one();
            for j in 0..m {
                let mut t = list[(k + j + m) as usize];
                t *= &w;
                let mut tmp = list[(k + j) as usize];
                tmp -= &t;
                list[(k + j + m) as usize] = tmp;
                list[(k + j) as usize] += &t;
                w.mul_assign(&w_m);
            }

            k += 2 * m;
        }

        m *= 2;
    }
}

fn bitreverse(mut n: u32, l: u32) -> u32 {
    let mut r = 0;
    for _ in 0..l {
        r = (r << 1) | (n & 1);
        n >>= 1;
    }
    r
}
