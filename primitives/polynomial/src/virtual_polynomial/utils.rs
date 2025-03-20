use crate::multilinear::Multilinear;
use ark_ff::PrimeField;
use ark_std::rand::RngCore;

pub fn random_mle_list<F: PrimeField, R: RngCore>(
    nv: usize,
    degree: usize,
    rng: &mut R,
) -> (Vec<Multilinear<F>>, F) {
    let mut multiplicands = Vec::with_capacity(degree);
    for _ in 0..degree {
        multiplicands.push(Vec::with_capacity(1 << nv))
    }
    let mut sum = F::zero();

    for _ in 0..(1 << nv) {
        let mut product = F::one();

        for e in multiplicands.iter_mut() {
            let val = F::rand(rng);
            e.push(val);
            product *= val;
        }
        sum += product;
    }

    let list = multiplicands
        .into_iter()
        .map(|x| Multilinear::new(x, nv))
        .collect();

    (list, sum)
}

pub fn random_zero_mle_list<F: PrimeField, R: RngCore>(
    nv: usize,
    degree: usize,
    rng: &mut R,
) -> Vec<Multilinear<F>> {
    let mut multiplicands = Vec::with_capacity(degree);
    for _ in 0..degree {
        multiplicands.push(Vec::with_capacity(1 << nv))
    }
    for _ in 0..(1 << nv) {
        multiplicands[0].push(F::zero());
        for e in multiplicands.iter_mut().skip(1) {
            e.push(F::rand(rng));
        }
    }

    let list = multiplicands
        .into_iter()
        .map(|x| Multilinear::new(x, nv))
        .collect();

    list
}

pub fn build_eq_x_r<F: PrimeField>(r: &[F]) -> Result<Multilinear<F>, anyhow::Error> {
    let evals = build_eq_x_r_vec(r)?;
    let mle = Multilinear::new(evals, r.len());

    Ok(mle)
}

pub fn build_eq_x_r_vec<F: PrimeField>(r: &[F]) -> Result<Vec<F>, anyhow::Error> {
    let mut eval = Vec::new();
    build_eq_x_r_helper(r, &mut eval)?;

    Ok(eval)
}

fn build_eq_x_r_helper<F: PrimeField>(r: &[F], buf: &mut Vec<F>) -> Result<(), anyhow::Error> {
    if r.is_empty() {
        return Err(anyhow::Error::msg("r length is 0"));
    } else if r.len() == 1 {
        // initializing the buffer with [1-r_0, r_0]
        buf.push(F::one() - r[0]);
        buf.push(r[0]);
    } else {
        build_eq_x_r_helper(&r[1..], buf)?;

        let mut res = vec![F::zero(); buf.len() << 1];
        res.iter_mut().enumerate().for_each(|(i, val)| {
            let bi = buf[i >> 1];
            let tmp = r[0] * bi;
            if i & 1 == 0 {
                *val = bi - tmp;
            } else {
                *val = tmp;
            }
        });
        *buf = res;
    }

    Ok(())
}
