//! This is another poly structure whose need rose from the implementation of the product check IOP
//! we need a way to represent a multivariate polynomial of this structure: f=c0​⋅f0​⋅f1​⋅f2​+c1​⋅f3​⋅f4
//! for better understand, see: https://github.com/EspressoSystems/hyperplonk/blob/main/arithmetic/src/virtual_polynomial.rs

use crate::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};
use anyhow::anyhow;
use ark_ff::PrimeField;
use ark_std::rand::{Rng, RngCore};
use std::{cmp::max, collections::HashMap, ops::Add, sync::Arc};
use utils::{build_eq_x_r, random_mle_list, random_zero_mle_list};
pub mod utils;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualPolynomial<F: PrimeField> {
    pub num_var: usize,
    pub max_degree: usize,
    pub products: Vec<(F, Vec<usize>)>,
    pub flattened_multilinear_poly: Vec<Arc<Multilinear<F>>>,
    raw_pointers_lookup_table: HashMap<*const Multilinear<F>, usize>,
}

impl<F: PrimeField> Add for &VirtualPolynomial<F> {
    type Output = VirtualPolynomial<F>;
    fn add(self, other: &VirtualPolynomial<F>) -> Self::Output {
        let mut res = self.clone();
        for products in other.products.iter() {
            let cur: Vec<Arc<Multilinear<F>>> = products
                .1
                .iter()
                .map(|&x| other.flattened_multilinear_poly[x].clone())
                .collect();

            res.add_mle_list(cur, products.0)
                .expect("add product failed");
        }
        res
    }
}

impl<F: PrimeField> VirtualPolynomial<F> {
    pub fn new(num_var: usize) -> Self {
        Self {
            num_var,
            max_degree: 0,
            products: Vec::new(),
            flattened_multilinear_poly: Vec::new(),
            raw_pointers_lookup_table: HashMap::new(),
        }
    }

    pub fn new_from_mle(mle: &Arc<Multilinear<F>>, coefficient: F) -> Self {
        let mle_ptr: *const Multilinear<F> = Arc::as_ptr(mle);
        let mut hm = HashMap::new();
        hm.insert(mle_ptr, 0);

        VirtualPolynomial {
            num_var: mle.num_vars,
            max_degree: 1,
            products: vec![(coefficient, vec![0])],
            flattened_multilinear_poly: vec![mle.clone()],
            raw_pointers_lookup_table: hm,
        }
    }

    pub fn add_mle_list(
        &mut self,
        mle_list: impl IntoIterator<Item = Arc<Multilinear<F>>>,
        coefficient: F,
    ) -> Result<(), anyhow::Error> {
        let mle_list: Vec<Arc<Multilinear<F>>> = mle_list.into_iter().collect();
        let mut indexed_product = Vec::with_capacity(mle_list.len());

        if mle_list.is_empty() {
            return Err(anyhow!("input mle_list is empty"));
        }

        self.max_degree = max(self.max_degree, mle_list.len());

        for mle in mle_list {
            if mle.num_vars != self.num_var {
                return Err(anyhow!(
                    "product has a multiplicand with wrong number of variables {} vs {}",
                    mle.num_vars,
                    self.num_var
                ));
            }

            let mle_ptr: *const Multilinear<F> = Arc::as_ptr(&mle);
            if let Some(index) = self.raw_pointers_lookup_table.get(&mle_ptr) {
                println!("No need to push: {}", index);
                indexed_product.push(*index)
            } else {
                let curr_index = self.flattened_multilinear_poly.len();
                self.flattened_multilinear_poly.push(mle.clone());
                self.raw_pointers_lookup_table.insert(mle_ptr, curr_index);
                indexed_product.push(curr_index);
            }
        }
        self.products.push((coefficient, indexed_product));
        Ok(())
    }

    pub fn mul_by_mle(
        &mut self,
        mle: Arc<Multilinear<F>>,
        coefficient: F,
    ) -> Result<(), anyhow::Error> {
        if mle.num_vars != self.num_var {
            return Err(anyhow!(
                "product has a multiplicand with wrong number of variables"
            ));
        }

        let mle_ptr: *const Multilinear<F> = Arc::as_ptr(&mle);

        // check if this mle already exists in the virtual polynomial
        let mle_index = match self.raw_pointers_lookup_table.get(&mle_ptr) {
            Some(&p) => p,
            None => {
                self.raw_pointers_lookup_table
                    .insert(mle_ptr, self.flattened_multilinear_poly.len());
                self.flattened_multilinear_poly.push(mle);
                self.flattened_multilinear_poly.len() - 1
            }
        };

        for (prod_coef, indices) in self.products.iter_mut() {
            // - add the MLE to the MLE list;
            // - multiple each product by MLE and its coefficient.
            indices.push(mle_index);
            *prod_coef *= coefficient;
        }

        // increase the max degree by one as the MLE has degree 1.
        self.max_degree += 1;

        Ok(())
    }

    pub fn evaluate(&self, point: &[F]) -> Result<F, anyhow::Error> {
        if self.num_var != point.len() {
            return Err(anyhow!("wrong number of variables"));
        }

        let evals: Vec<F> = self
            .flattened_multilinear_poly
            .iter()
            .map(|x| x.evaluate(&point.to_vec()).unwrap())
            .collect();

        let res = self
            .products
            .iter()
            .map(|(c, p)| *c * p.iter().map(|&i| evals[i]).product::<F>())
            .sum();

        Ok(res)
    }

    pub fn rand<R: RngCore>(
        nv: usize,
        num_multiplicands_range: (usize, usize),
        num_products: usize,
        rng: &mut R,
    ) -> Result<(Self, F), anyhow::Error> {
        let mut sum = F::zero();
        let mut poly = VirtualPolynomial::new(nv);
        for _ in 0..num_products {
            let num_multiplicands =
                rng.gen_range(num_multiplicands_range.0..num_multiplicands_range.1);
            let (product, product_sum) = random_mle_list(nv, num_multiplicands, rng);
            let coefficient = F::rand(rng);
            poly.add_mle_list(product.into_iter(), coefficient)?;
            sum += product_sum * coefficient;
        }

        Ok((poly, sum))
    }

    pub fn rand_zero<R: RngCore>(
        nv: usize,
        num_multiplicands_range: (usize, usize),
        num_products: usize,
        rng: &mut R,
    ) -> Result<Self, anyhow::Error> {
        let mut poly = VirtualPolynomial::new(nv);
        for _ in 0..num_products {
            let num_multiplicands =
                rng.gen_range(num_multiplicands_range.0..num_multiplicands_range.1);
            let product = random_zero_mle_list(nv, num_multiplicands, rng);
            let coefficient = F::rand(rng);
            poly.add_mle_list(product.into_iter(), coefficient)?;
        }

        Ok(poly)
    }

    pub fn build_f_hat(&self, r: &[F]) -> Result<Self, anyhow::Error> {
        if self.num_var != r.len() {
            return Err(anyhow::Error::msg(
                "r.len() is different from number of variables",
            ));
        }

        let eq_x_r = build_eq_x_r(r)?;
        let mut res = self.clone();
        res.mul_by_mle(eq_x_r, F::one())?;

        Ok(res)
    }

    pub fn to_bytes_serialized(&self) -> Vec<u8> {
        let mut bytes = vec![];

        for poly in &self.flattened_multilinear_poly {
            bytes.extend_from_slice(&poly.to_bytes());
        }

        bytes.extend_from_slice(&self.num_var.to_le_bytes());
        bytes.extend_from_slice(&self.max_degree.to_le_bytes());
        bytes.extend_from_slice(&self.products.len().to_le_bytes());
        bytes.extend_from_slice(&self.flattened_multilinear_poly.len().to_le_bytes());
        bytes.extend_from_slice(&self.raw_pointers_lookup_table.len().to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_ff::UniformRand;
    use ark_std::test_rng;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_virtual_polynomial_additions() -> Result<(), anyhow::Error> {
        let mut rng = test_rng();
        for nv in 2..5 {
            for num_products in 2..5 {
                let base: Vec<Fr> = (0..nv).map(|_| Fr::rand(&mut rng)).collect();

                let (a, _a_sum) =
                    VirtualPolynomial::<Fr>::rand(nv, (2, 3), num_products, &mut rng)?;
                let (b, _b_sum) =
                    VirtualPolynomial::<Fr>::rand(nv, (2, 3), num_products, &mut rng)?;
                let c = &a + &b;

                assert_eq!(
                    a.evaluate(base.as_ref())? + b.evaluate(base.as_ref())?,
                    c.evaluate(base.as_ref())?
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_virtual_polynomial_mul_by_mle() -> Result<(), anyhow::Error> {
        let mut rng = test_rng();
        for nv in 2..5 {
            for num_products in 2..5 {
                let base: Vec<Fr> = (0..nv).map(|_| Fr::rand(&mut rng)).collect();

                let (a, _a_sum) =
                    VirtualPolynomial::<Fr>::rand(nv, (2, 3), num_products, &mut rng)?;
                let (b, _b_sum) = random_mle_list(nv, 1, &mut rng);
                let b_mle = b[0].clone();
                let coeff = Fr::rand(&mut rng);
                let b_vp = VirtualPolynomial::new_from_mle(&b_mle, coeff);

                let mut c = a.clone();

                c.mul_by_mle(b_mle, coeff)?;

                assert_eq!(
                    a.evaluate(base.as_ref())? * b_vp.evaluate(base.as_ref())?,
                    c.evaluate(base.as_ref())?
                );
            }
        }

        Ok(())
    }

    #[test]
    fn test_eq_xr() {
        let mut rng = test_rng();
        for nv in 4..10 {
            let r: Vec<Fr> = (0..nv).map(|_| Fr::rand(&mut rng)).collect();
            let eq_x_r = build_eq_x_r(r.as_ref()).unwrap();
            let eq_x_r2 = build_eq_x_r_for_test(r.as_ref());
            assert_eq!(eq_x_r, eq_x_r2);

            break;
        }
    }

    /// Naive method to build eq(x, r).
    /// Only used for testing purpose.
    // Evaluate
    //      eq(x,y) = \prod_i=1^num_var (x_i * y_i + (1-x_i)*(1-y_i))
    // over r, which is
    //      eq(x,y) = \prod_i=1^num_var (x_i * r_i + (1-x_i)*(1-r_i))
    fn build_eq_x_r_for_test<F: PrimeField>(r: &[F]) -> Arc<Multilinear<F>> {
        // we build eq(x,r) from its evaluations
        // we want to evaluate eq(x,r) over x \in {0, 1}^num_vars
        // for example, with num_vars = 4, x is a binary vector of 4, then
        //  0 0 0 0 -> (1-r0)   * (1-r1)    * (1-r2)    * (1-r3)
        //  1 0 0 0 -> r0       * (1-r1)    * (1-r2)    * (1-r3)
        //  0 1 0 0 -> (1-r0)   * r1        * (1-r2)    * (1-r3)
        //  1 1 0 0 -> r0       * r1        * (1-r2)    * (1-r3)
        //  ....
        //  1 1 1 1 -> r0       * r1        * r2        * r3
        // we will need 2^num_var evaluations

        println!("These are rs: {:?}", r);

        // First, we build array for {1 - r_i}
        let one_minus_r: Vec<F> = r.iter().map(|ri| F::one() - ri).collect();

        let num_var = r.len();
        let mut eval = vec![];

        for i in 0..1 << num_var {
            let mut current_eval = F::one();
            let bit_sequence = bit_decompose(i, num_var);

            for (&bit, (ri, one_minus_ri)) in
                bit_sequence.iter().zip(r.iter().zip(one_minus_r.iter()))
            {
                current_eval *= if bit { *ri } else { *one_minus_ri };
            }
            eval.push(current_eval);
        }

        let mle = Multilinear::new(eval, num_var);

        Arc::new(mle)
    }

    pub fn bit_decompose(input: u64, num_var: usize) -> Vec<bool> {
        let mut res = Vec::with_capacity(num_var);
        let mut i = input;
        for _ in 0..num_var {
            res.push(i & 1 == 1);
            i >>= 1;
        }
        res
    }
}
