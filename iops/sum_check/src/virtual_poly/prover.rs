//! Prover module for the sumcheck protocol of the virtual polynomial.
use std::sync::Arc;

use super::{SumCheckProver, SumCheckProverMessage};
use ark_ff::{batch_inversion, PrimeField};
use ark_std::cfg_into_iter;
use polynomial::{multilinear::Multilinear, virtual_polynomial::VirtualPolynomial};
use rayon::iter::{IntoParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

#[derive(Clone, Default, Debug)]
pub struct VirtualProver<F: PrimeField> {
    /// sampled randomness given by the verifier
    pub challenges: Vec<F>,
    /// the current round number
    pub(crate) round: usize,
    /// pointer to the virtual polynomial
    pub(crate) poly: VirtualPolynomial<F>,
    /// points with precomputed barycentric weights for extrapolating smaller
    /// degree uni-polys to `max_degree + 1` evaluations.
    pub(crate) extrapolation_aux: Vec<(Vec<F>, Vec<F>)>,
}

impl<F: PrimeField> SumCheckProver<F> for VirtualProver<F> {
    type VirtualPolynomial = VirtualPolynomial<F>;
    type ProverMessage = SumCheckProverMessage<F>;

    fn prover_init(polynomial: &Self::VirtualPolynomial) -> Result<Self, anyhow::Error> {
        if polynomial.num_var == 0 {
            return Err(anyhow::anyhow!(
                "polynomial must have at least one variable"
            ));
        }

        Ok(Self {
            challenges: Vec::with_capacity(polynomial.num_var),
            round: 0,
            poly: polynomial.clone(),
            extrapolation_aux: (1..polynomial.max_degree)
                .map(|degree| {
                    let points = (0..1 + degree as u64).map(F::from).collect::<Vec<_>>();
                    let weights = barycentric_weights(&points);
                    (points, weights)
                })
                .collect(),
        })
    }

    fn prove_round_and_update_state(
        &mut self,
        challenge: &Option<F>,
    ) -> Result<Self::ProverMessage, anyhow::Error> {
        if self.round >= self.poly.num_var {
            return Err(anyhow::anyhow!("Prover is not active"));
        }

        let mut flattened_multilinear_poly: Vec<Multilinear<F>> = self
            .poly
            .flattened_multilinear_poly
            .iter()
            .map(|x| x.as_ref().clone())
            .collect();

        if let Some(chal) = challenge {
            if self.round == 0 {
                return Err(anyhow::anyhow!("first round should be prover first."));
            }
            self.challenges.push(*chal);

            let r = self.challenges[self.round - 1];
            flattened_multilinear_poly
                .iter_mut()
                .for_each(|mle| *mle = fix_variables(mle, &[r]));
        } else if self.round > 0 {
            return Err(anyhow::anyhow!("verifier message is empty"));
        }

        self.round += 1;

        let products_list = self.poly.products.clone();
        let mut products_sum = vec![F::zero(); self.poly.max_degree + 1];

        products_list.iter().for_each(|(coefficient, products)| {
            let mut sum = ParallelIterator::fold(
                (0..1 << (self.poly.num_var - self.round)).into_par_iter(),
                || {
                    (
                        vec![(F::zero(), F::zero()); products.len()],
                        vec![F::zero(); products.len() + 1],
                    )
                },
                |(mut buf, mut acc), b| {
                    buf.iter_mut()
                        .zip(products.iter())
                        .for_each(|((eval, step), f)| {
                            let table = &flattened_multilinear_poly[*f];
                            *eval = table.evaluations[b << 1];
                            *step = table.evaluations[(b << 1) + 1] - table.evaluations[b << 1];
                        });
                    acc[0] += buf.iter().map(|(eval, _)| eval).product::<F>();
                    acc[1..].iter_mut().for_each(|acc| {
                        buf.iter_mut().for_each(|(eval, step)| *eval += step as &_);
                        *acc += buf.iter().map(|(eval, _)| eval).product::<F>();
                    });
                    (buf, acc)
                },
            )
            .map(|(_, partial)| partial)
            .reduce(
                || vec![F::zero(); products.len() + 1],
                |mut sum, partial| {
                    sum.iter_mut()
                        .zip(partial.iter())
                        .for_each(|(sum, partial)| *sum += partial);
                    sum
                },
            );
            sum.iter_mut().for_each(|sum| *sum *= coefficient);
            let extraploation = (0..self.poly.max_degree - products.len())
                .into_par_iter()
                .map(|i| {
                    let (points, weights) = &self.extrapolation_aux[products.len() - 1];
                    let at = F::from((products.len() + 1 + i) as u64);
                    extrapolate(points, weights, &sum, &at)
                })
                .collect::<Vec<_>>();
            products_sum
                .iter_mut()
                .zip(sum.iter().chain(extraploation.iter()))
                .for_each(|(products_sum, sum)| *products_sum += sum);
        });

        // update prover's state to the partial evaluated polynomial
        self.poly.flattened_multilinear_poly = flattened_multilinear_poly
            .iter()
            .map(|x| Arc::new(x.clone()))
            .collect();

        Ok(SumCheckProverMessage {
            evaluations: products_sum,
        })
    }
}

fn barycentric_weights<F: PrimeField>(points: &[F]) -> Vec<F> {
    let mut weights = points
        .iter()
        .enumerate()
        .map(|(j, point_j)| {
            points
                .iter()
                .enumerate()
                .filter(|&(i, _point_i)| (i != j))
                .map(|(_i, point_i)| *point_j - point_i)
                .reduce(|acc, value| acc * value)
                .unwrap_or_else(F::one)
        })
        .collect::<Vec<_>>();
    batch_inversion(&mut weights);
    weights
}

pub fn fix_variables<F: PrimeField>(poly: &Multilinear<F>, partial_point: &[F]) -> Multilinear<F> {
    assert!(
        partial_point.len() <= poly.num_vars,
        "invalid size of partial point"
    );
    let nv = poly.num_vars;
    let mut poly = poly.evaluations.to_vec();
    let dim = partial_point.len();
    // evaluate single variable of partial point from left to right
    for (i, point) in partial_point.iter().enumerate().take(dim) {
        poly = fix_one_variable_helper(&poly, nv - i, point);
    }

    Multilinear::<F>::new(poly[..(1 << (nv - dim))].to_vec(), nv - dim)
}

fn fix_one_variable_helper<F: PrimeField>(data: &[F], nv: usize, point: &F) -> Vec<F> {
    let mut res = vec![F::zero(); 1 << (nv - 1)];

    // evaluate single variable of partial point from left to right
    for i in 0..(1 << (nv - 1)) {
        res[i] = data[i] + (data[(i << 1) + 1] - data[i << 1]) * point;
    }

    res
}

fn extrapolate<F: PrimeField>(points: &[F], weights: &[F], evals: &[F], at: &F) -> F {
    let (coeffs, sum_inv) = {
        let mut coeffs = points.iter().map(|point| *at - point).collect::<Vec<_>>();
        batch_inversion(&mut coeffs);
        coeffs.iter_mut().zip(weights).for_each(|(coeff, weight)| {
            *coeff *= weight;
        });
        let sum_inv = coeffs.iter().sum::<F>().inverse().unwrap_or_default();
        (coeffs, sum_inv)
    };
    coeffs
        .iter()
        .zip(evals)
        .map(|(coeff, eval)| *coeff * eval)
        .sum::<F>()
        * sum_inv
}
