//! This is another poly structure whose need rose from the implementation of the product check IOP
//! we need a way to represent a multivariate polynomial of this structure: f=c0​⋅f0​⋅f1​⋅f2​+c1​⋅f3​⋅f4
//! for better understand, see: https://github.com/EspressoSystems/hyperplonk/blob/main/arithmetic/src/virtual_polynomial.rs

use crate::{interface::MultilinearPolynomialInterface, multilinear::Multilinear};
use anyhow::anyhow;
use ark_ff::PrimeField;
use ark_std::rand::{Rng, RngCore};
use std::{cmp::max, collections::HashMap, ops::Add};
use utils::{build_eq_x_r, random_mle_list, random_zero_mle_list};
pub mod utils;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualPolynomial<F: PrimeField> {
    pub num_var: usize,
    pub max_degree: usize,
    pub products: Vec<(F, Vec<usize>)>,
    pub flattened_multilinear_poly: Vec<Multilinear<F>>,
    raw_pointers_lookup_table: HashMap<*const Multilinear<F>, usize>,
}

impl<F: PrimeField> Add for &VirtualPolynomial<F> {
    type Output = VirtualPolynomial<F>;
    fn add(self, other: &VirtualPolynomial<F>) -> Self::Output {
        let mut res = self.clone();
        for products in other.products.iter() {
            let cur: Vec<Multilinear<F>> = products
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

    pub fn new_from_mle(mle: &Multilinear<F>, coefficient: F) -> Self {
        let mle_ptr = mle as *const Multilinear<F>;
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
        mle_list: impl IntoIterator<Item = Multilinear<F>>,
        coefficient: F,
    ) -> Result<(), anyhow::Error> {
        let mle_list: Vec<Multilinear<F>> = mle_list.into_iter().collect();
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

            let mle_ptr: *const Multilinear<F> = &mle;
            if let Some(index) = self.raw_pointers_lookup_table.get(&mle_ptr) {
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

    pub fn mul_by_mle(&mut self, mle: Multilinear<F>, coefficient: F) -> Result<(), anyhow::Error> {
        if mle.num_vars != self.num_var {
            return Err(anyhow!(
                "product has a multiplicand with wrong number of variables"
            ));
        }

        let mle_ptr: *const Multilinear<F> = &mle;

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

    fn to_bytes(&self) -> Vec<u8> {
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
