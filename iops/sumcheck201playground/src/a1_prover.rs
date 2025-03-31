use crate::{
    primitives::{LinearLagrangeList, ProverState, SumcheckProof},
    transcript::TranscriptProtocol,
};
use ark_ff::PrimeField;
use ark_std::log2;
use fiat_shamir::FiatShamirTranscript;
use std::marker::PhantomData;

/// Interactive Proof for Multilinear Sumcheck
/// Same as arkworks ML sumcheck implementation
pub struct IPForMLSumcheck<F: PrimeField> {
    #[doc(hidden)]
    _marker: PhantomData<F>,
}

impl<F: PrimeField> IPForMLSumcheck<F> {
    /// Initialise prover state from a given set of polynomials (in their evaluation form).
    /// The degree of the sumcheck round polynomial also needs to be input.
    pub fn prover_init(
        polynomials: &Vec<LinearLagrangeList<F>>,
        sumcheck_poly_degree: usize,
    ) -> ProverState<F> {
        // sanity check 1: no polynomials case must not be allowed.
        if polynomials.len() == 0 {
            panic!("Cannot prove empty input polynomials.")
        }

        // sanity check 2: all polynomial evaluations must be of the same size.
        let problem_size = polynomials[0].size;
        let _ = polynomials.iter().enumerate().map(|(i, poly)| {
            if poly.size != problem_size {
                panic!("Polynomial size mismatch at {}", i)
            }
        });

        // sanity check 3: size must be a power of two.
        if !problem_size.is_power_of_two() {
            panic!("Number of polynomial evaluations must be a power of two.")
        }

        let num_variables: usize = log2(2 * problem_size).try_into().unwrap();
        ProverState {
            randomness: Vec::with_capacity(num_variables),
            state_polynomials: polynomials.to_vec(),
            num_vars: num_variables,
            max_multiplicands: sumcheck_poly_degree,
            round: 0,
        }
    }

    ///
    /// Creates a sumcheck proof consisting of `n` round polynomials each of degree `d` using Algorithm 1.
    /// We allow for any function `combine_function` on a set of MLE polynomials.
    ///
    pub fn prove<C>(
        prover_state: &mut ProverState<F>,
        combine_function: &C,
        transcript: &mut FiatShamirTranscript,
    ) -> SumcheckProof<F>
    where
        C: Fn(&Vec<F>) -> F + Sync,
    {
        // Initiate the transcript with the protocol name
        <FiatShamirTranscript as TranscriptProtocol<F>>::sumcheck_proof_domain_sep(
            transcript,
            prover_state.num_vars as u64,
            prover_state.max_multiplicands as u64,
        );

        // Declare r_polys and initialise it with 0s
        let r_degree = prover_state.max_multiplicands;
        let mut r_polys: Vec<Vec<F>> = (0..prover_state.num_vars)
            .map(|_| vec![F::zero(); r_degree + 1])
            .collect();

        for round_index in 0..prover_state.num_vars {
            let state_polynomial_len = prover_state.state_polynomials[0].list.len();
            for k in 0..(r_degree + 1) {
                for i in 0..state_polynomial_len {
                    let evaluations_at_k = prover_state
                        .state_polynomials
                        .iter()
                        .map(|state_poly| {
                            // evaluate given state polynomial at x_1 = k
                            let o = state_poly.list[i].odd;
                            let e = state_poly.list[i].even;
                            (F::one() - F::from(k as u32)) * e + F::from(k as u32) * o
                        })
                        .collect::<Vec<F>>();

                    // apply combine function
                    r_polys[round_index][k] += combine_function(&evaluations_at_k);
                }
            }

            // append the round polynomial (i.e. prover message) to the transcript
            <FiatShamirTranscript as TranscriptProtocol<F>>::append_scalars(
                transcript,
                "r_poly",
                &r_polys[round_index],
            );

            // generate challenge α_i = H( transcript );
            let alpha = <FiatShamirTranscript as TranscriptProtocol<F>>::challenge_scalar(
                transcript,
                b"challenge_nextround",
            );

            // update prover state polynomials
            for j in 0..prover_state.state_polynomials.len() {
                prover_state.state_polynomials[j].fold_in_half(alpha);
            }
        }

        SumcheckProof {
            num_vars: prover_state.num_vars,
            degree: r_degree,
            round_polynomials: r_polys,
        }
    }
}
