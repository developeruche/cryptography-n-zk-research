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

pub type PartialSumCheckProof<F, E> = (Vec<Vec<Fields<F, E>>>, Vec<E>);

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
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error>;
}

/// A struct for a linear time sum check protocol.
pub struct LinearTimeSumCheck<F: Field, E: ExtensionField<F>> {
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
    ) -> Result<PartialSumCheckProof<F, E>, anyhow::Error> {
        let (phase_one_round_polys, u) = Self::phase_one(f_1, f_2_3, i_gz, transcript)?;

        let i_uz = generate_igz::<F, E>(&u);
        let f2_vec = &f_2_3.to_vec();
        let f2: MultilinearPoly<F, E> = f2_vec.into();
        let f2_u = f2.evaluate(&u.iter().map(|&x| Fields::Extension(x)).collect::<Vec<_>>());

        let (phase_two_round_polys, v) = Self::phase_two(
            f_1,
            f_2_3,
            i_gz,
            &i_uz,
            &f2_u.to_extension_field(),
            transcript,
        )?;

        Ok((
            [phase_one_round_polys, phase_two_round_polys].concat(),
            [u, v].concat(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use p3_field::{AbstractField, extension::BinomialExtensionField};
    use p3_mersenne_31::Mersenne31;
    use sum_check::primitives::SumCheckProof;

    use super::*;

    type F = Mersenne31;
    type E = BinomialExtensionField<Mersenne31, 3>;
    type Mle = VPoly<F, E>;

    /// Creates polynomial setup for testing sum-check protocols.
    /// Returns the v_poly for use in tests.
    fn create_test_polynomial_setup() -> VPoly<F, E> {
        let mut f_var_zero_vec = vec![Fields::<F, E>::Base(F::new(0u32)); 32];
        f_var_zero_vec[1] = Fields::<F, E>::Base(F::new(1u32));
        let f_1_trad = MultilinearPoly::new_from_vec(5, f_var_zero_vec);

        let g = Fields::<F, E>::Base(F::new(2u32));

        let f2_x_f2_y_vec = [1u32, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16]
            .iter()
            .map(|&x| Fields::<F, E>::Base(F::new(x)))
            .collect::<Vec<_>>();
        let f2_x_f2_y = MultilinearPoly::new_from_vec(4, f2_x_f2_y_vec);

        let f_1_trad_g = f_1_trad.partial_evaluate(&[g]);

        VPoly::<F, E>::new(
            vec![f_1_trad_g, f2_x_f2_y],
            2,
            4,
            Rc::new(product_combined_fn),
        )
    }

    #[test]
    fn test_traditional_sum_check() {
        // This is how this test would go;
        // Would be running sum_check traditionally then linear time
        // v_poly = f_1(g, x, y)f_2(x)f_2(y)
        //
        // f_1 = 5 var; [(0, 00, 01)] -> [0 00 00, 0 00 01(this only no-zero element), 0 00 10, 0 00 11 .... 1 11 11];
        // f_2 = 2 var; [1, 2, 3, 4]
        // f_2(x) * f_2(y) = [1, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16] (4 vars)
        //
        // we would partial evaluate f_1 at g, returning f_1(x, y)
        //
        // as this put we can run production sum check tradionally

        let v_poly = create_test_polynomial_setup();

        let claimed_sum = v_poly.sum_over_hypercube();

        let mut transcript = Transcript::init();
        let proof = SumCheck::prove(claimed_sum, &v_poly, &mut transcript).unwrap();

        let mut transcript = Transcript::init();
        let verified = SumCheck::verify(&v_poly, &proof, &mut transcript).unwrap();

        assert!(verified);
    }

    #[test]
    fn test_linear_time_sum_check() {
        // Silimilar to `test_traditional_sum_check`, we would be would on the sparse version of `f_1`
        // all other part of this example remains the same;

        let f_1_sparse = vec![(0, 0, 1)];

        // let f2_x_f2_y_vec = [1u32, 2, 3, 4, 2, 4, 6, 8, 3, 6, 9, 12, 4, 8, 12, 16].iter().map(|&x| F::new(x)).collect::<Vec<_>>();
        let f2_x_f2_y_vec = [1u32, 2, 3, 4]
            .iter()
            .map(|&x| F::new(x))
            .collect::<Vec<_>>();

        let g = E::from_wrapped_u32(2u32);

        let ig_z = generate_igz::<F, E>(&[g]);

        let mut transcript = Transcript::init();
        let (round_polys, challenges) =
            LinearTimeSumCheck::sum_check(&f_1_sparse, &f2_x_f2_y_vec, &ig_z, &mut transcript)
                .unwrap();

        let claimed_sum = Fields::Base(F::new(2147483645));

        let v_proof = SumCheckProof::new(claimed_sum, round_polys);

        let mut transcript = Transcript::init();

        let (cs, c) = SumCheck::<F, E, Mle>::verify_partial(&v_proof, &mut transcript);

        let v_poly = create_test_polynomial_setup();

        let c_p = challenges
            .iter()
            .map(|&x| Fields::<F, E>::Extension(x))
            .collect::<Vec<_>>();

        assert_eq!(c, c_p);

        assert_eq!(Fields::Extension(cs), v_poly.evaluate(&c));
    }
}
