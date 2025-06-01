//! This is an implemention of a linear time sum check protocol.
//! as discribed in the Libra paper.
use std::rc::Rc;
use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{Fields, vpoly::VPoly};
use sum_check::{SumCheck, interface::SumCheckInterface};
use transcript::Transcript;

use crate::{algos::initialize_phase_1, utils::product_combined_fn};

type PartialSumCheckProof<F, E> = (Vec<Vec<Fields<F, E>>>, Vec<E>);

/// A trait for a linear time sum check protocol.
pub trait LinearTimeSumCheckTr<F: Field + PrimeField32, E: ExtensionField<F>> {
    /// phase one houses two algorithms
    /// 1. initialize: Creating the A_h_g book-keeping table using Algorithm 4
    /// 2. sum_check_product: Performing the sum check product operation using the sl-core library
    ///
    /// params:
    /// f_1: &[(usize, usize, usize)] :> This is a sparse polynomial in GKR protocol, this could be the add_mle or mul_mle
    /// f_2_3: &[F] :> This is normal polynomial, in GKR protocol this could be the w_i_plus_one polynomial. (these two polynomials have the same structure but evaluated on different points-(x and y))
    /// i_gz: &[E] :> This is the i(g,z) table generated from `g`.
    fn phase_one(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        transcript: &mut Transcript<F, E>,
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error>;
    fn phase_two();
    fn sum_check();
}

/// A struct for a linear time sum check protocol.
pub(crate) struct LinearTimeSumCheck<F: Field, E: ExtensionField<F>> {
    _marker: std::marker::PhantomData<(F, E)>,
}

impl<F: Field + PrimeField32, E: ExtensionField<F>> LinearTimeSumCheckTr<F, E>
    for LinearTimeSumCheck<F, E>
{
    fn phase_one(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        transcript: &mut Transcript<F, E>,
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error> {
        let a_hg = initialize_phase_1(f_1, f_2_3, i_gz);
        let num_var = (a_hg.len() as f64).log2() as usize;
        let product_poly = VPoly::new(vec![], 2, num_var, Rc::new(product_combined_fn));

        SumCheck::prove_partial(&product_poly, transcript)
    }

    fn phase_two() {
        todo!()
    }

    fn sum_check() {
        todo!()
    }
}
