//! This is an implemention of a linear time sum check protocol.
//! as discribed in the Libra paper.
use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{Fields, MultilinearExtension, mle::MultilinearPoly, vpoly::VPoly};
use std::rc::Rc;
use sum_check::{SumCheck, interface::SumCheckInterface};
use transcript::Transcript;

use crate::{
    algos::{initialize_phase_1, initialize_phase_2},
    utils::{generate_igz, product_combined_fn},
};

type PartialSumCheckProof<F, E> = (Vec<Vec<Fields<F, E>>>, Vec<E>);
type SumCheckProof<F, E> = Vec<Vec<Fields<F, E>>>;

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

    /// phase two houses two algorithms
    /// 1. initialize: Creating the A_f_1 book-keeping table using Algorithm 5
    /// 2. sum_check_product: Performing the sum check product operation using the sl-core library
    ///
    /// params:
    /// f_1: &[(usize, usize, usize)] :> This is a sparse polynomial in GKR protocol, this could be the add_mle or mul_mle
    /// i_gz: &[E] :> This is the i(g,z) table generated from `g`.
    /// i_uz: &[E] :> This is the i(u,z) table generated from `u`.
    fn phase_two(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        i_uz: &[E],
        f2_u: &E,
        transcript: &mut Transcript<F, E>,
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error>;

    /// Sum-check a combination of two phases.
    fn sum_check(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        transcript: &mut Transcript<F, E>,
    ) -> Result<SumCheckProof<F, E>, anyhow::Error>;
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

        let a_hg_mle = MultilinearPoly::new_from_vec(
            num_var,
            a_hg.iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<_>>(),
        );
        let f_2_3_mle = MultilinearPoly::new_from_vec(
            num_var,
            f_2_3.iter().map(|&x| Fields::Base(x)).collect::<Vec<_>>(),
        );
        let product_poly = VPoly::new(
            vec![a_hg_mle, f_2_3_mle],
            2,
            num_var,
            Rc::new(product_combined_fn),
        );

        SumCheck::prove_partial(&product_poly, transcript)
    }

    fn phase_two(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        i_uz: &[E],
        f2_u: &E,
        transcript: &mut Transcript<F, E>,
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error> {
        let a_f_1 = initialize_phase_2(f_1, i_gz, i_uz);
        let num_var = (a_f_1.len() as f64).log2() as usize;

        let f_2_3_vec = &f_2_3.to_vec();
        let f_2_3_mle: MultilinearPoly<F, E> = f_2_3_vec.into();
        let f3_y_mul_f2_u = f_2_3_mle * Fields::<F, E>::Extension(*f2_u);
        let product_poly = VPoly::new(
            vec![a_f_1.into(), f3_y_mul_f2_u],
            2,
            num_var,
            Rc::new(product_combined_fn),
        );

        SumCheck::prove_partial(&product_poly, transcript)
    }

    fn sum_check(
        f_1: &[(usize, usize, usize)],
        f_2_3: &[F],
        i_gz: &[E],
        transcript: &mut Transcript<F, E>,
    ) -> Result<SumCheckProof<F, E>, anyhow::Error> {
        let (phase_one_round_polys, u) = Self::phase_one(f_1, f_2_3, i_gz, transcript)?;

        let i_uz = generate_igz::<F, E>(&u);
        let f2_vec = &f_2_3.to_vec();
        let f2: MultilinearPoly<F, E> = f2_vec.into();
        let f2_u = f2.evaluate(&u.iter().map(|&x| Fields::Extension(x)).collect::<Vec<_>>());

        let (phase_two_round_polys, _) = Self::phase_two(
            f_1,
            f_2_3,
            i_gz,
            &i_uz,
            &f2_u.to_extension_field(),
            transcript,
        )?;

        Ok([phase_one_round_polys, phase_two_round_polys].concat())
    }
}
