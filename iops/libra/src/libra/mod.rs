//! An implementation of the Libra protocol.
use circuits::{
    interface::LibraGKRLayeredCircuitTr,
    layered_circuit::{LayeredCircuit, primitives::Evaluation},
};
use p3_field::{ExtensionField, Field, PrimeField32};
use poly::{Fields, MultilinearExtension, mle::MultilinearPoly};
use primitives::LibraProof;
use transcript::Transcript;

use crate::utils::{generate_igz, perform_libra_sumcheck};
pub mod primitives;

/// Interface for the Libra protocol.
pub trait LibraTr<F: Field + PrimeField32, E: ExtensionField<F>> {
    fn prove(
        circuit: &LayeredCircuit,
        output: Evaluation<F>,
    ) -> Result<LibraProof<F, E>, anyhow::Error>;
    fn verify(
        circuit: &LayeredCircuit,
        proofs: LibraProof<F, E>,
        input: Vec<F>,
    ) -> Result<bool, anyhow::Error>;
}

/// A struct representing the Libra protocol.
pub struct Libra<F: Field, E: ExtensionField<F>> {
    _marker: std::marker::PhantomData<(F, E)>,
}

impl<F: Field + PrimeField32, E: ExtensionField<F>> LibraTr<F, E> for Libra<F, E> {
    fn prove(
        circuit: &LayeredCircuit,
        output: Evaluation<F>,
    ) -> Result<LibraProof<F, E>, anyhow::Error> {
        // Initialize prover transcript
        let mut transcript = Transcript::<F, E>::init();
        let mut sumcheck_proofs = vec![];

        let mut wb_s_add_x = vec![];
        let mut wc_s_add_x = vec![];
        let mut wb_s_add_y = vec![];
        let mut wc_s_add_y = vec![];
        let mut wb_s_mul = vec![];
        let mut wc_s_mul = vec![];

        let mut last_rand_add_i_x_b;
        let mut last_rand_add_i_x_c;
        let mut last_rand_add_i_y_b;
        let mut last_rand_add_i_y_c;
        let mut last_rand_mul_i_b;
        let mut last_rand_mul_i_c;

        let mut last_alpha;
        let mut last_beta;

        // Get the output vector
        let mut output_evals: Vec<Fields<F, E>> = output.layers[circuit.layers.len()]
            .iter()
            .map(|val| Fields::<F, E>::Base(*val))
            .collect();

        if output_evals.len() == 1 {
            output_evals.push(Fields::Base(F::zero()));
        }

        // Build the output polynomial
        let output_mle = MultilinearPoly::new_from_vec(
            (output_evals.len() as f64).log2() as usize,
            output_evals,
        );

        // Adds the output to the transcript
        transcript.observe_base_element(&output.layers[circuit.layers.len()]);

        // Gets the addi and muli for the output layer
        let (add_i, mul_i) =
            LibraGKRLayeredCircuitTr::<F, E>::add_and_mul_mle(circuit, circuit.layers.len() - 1);

        // Gets w_i+1
        let mut w_i_plus_one = output.layers[circuit.layers.len() - 1].clone();

        let mut w_i_plus_one_iden;

        // Sample random challenge for the first round
        let g = transcript.sample_n_challenges(output_mle.num_vars());
        let mut i_gz = generate_igz(&g);

        let mut claimed_sum = output_mle.evaluate(
            &g.iter()
                .map(|val| Fields::Extension(*val))
                .collect::<Vec<Fields<F, E>>>(),
        );

        // at this point we've got all we need to run the 3 sum check protocols
        w_i_plus_one_iden = vec![F::from_canonical_u32(1); w_i_plus_one.len()];
        let (sum_check_proof, sum_check_challenges) = perform_libra_sumcheck(
            &add_i,
            &mul_i,
            &w_i_plus_one,
            &w_i_plus_one_iden,
            &i_gz,
            &claimed_sum,
            &mut transcript,
        )?;

        //TODO: append the proof to the transcript
        sumcheck_proofs.push(sum_check_proof);

        (last_rand_add_i_x_b, last_rand_add_i_x_c) = sum_check_challenges
            .add_i_x_challenges
            .split_at(sum_check_challenges.add_i_x_challenges.len() / 2);
        (last_rand_add_i_y_b, last_rand_add_i_y_c) = sum_check_challenges
            .add_i_y_challenges
            .split_at(sum_check_challenges.add_i_y_challenges.len() / 2);
        (last_rand_mul_i_b, last_rand_mul_i_c) = sum_check_challenges
            .mul_i_challenges
            .split_at(sum_check_challenges.mul_i_challenges.len() / 2);

        let w_i_plus_one_dereference = &w_i_plus_one;
        let w_i_plus_one_mle: MultilinearPoly<F, E> = w_i_plus_one_dereference.into();

        let eval_wb_s_add_x = w_i_plus_one_mle.evaluate(
            &last_rand_add_i_x_b
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wb_s_add_x.push(eval_wb_s_add_x);
        let eval_wc_s_add_x = w_i_plus_one_mle.evaluate(
            &last_rand_add_i_x_c
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wc_s_add_x.push(eval_wc_s_add_x);
        let eval_wb_s_add_y = w_i_plus_one_mle.evaluate(
            &last_rand_add_i_y_b
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wb_s_add_y.push(eval_wb_s_add_y);
        let eval_wc_s_add_y = w_i_plus_one_mle.evaluate(
            &last_rand_add_i_y_c
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wc_s_add_y.push(eval_wc_s_add_y);
        let eval_wb_s_mul = w_i_plus_one_mle.evaluate(
            &last_rand_mul_i_b
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wb_s_mul.push(eval_wb_s_mul);
        let eval_wc_s_mul = w_i_plus_one_mle.evaluate(
            &last_rand_mul_i_c
                .iter()
                .map(|&x| Fields::Extension(x))
                .collect::<Vec<Fields<F, E>>>(),
        );
        wc_s_mul.push(eval_wc_s_mul);

        last_alpha = transcript.sample_challenge();
        last_beta = transcript.sample_challenge();

        claimed_sum = (Fields::Extension(last_alpha) * eval_wb_s_add_x
            + Fields::Extension(last_beta) * eval_wc_s_add_x)
            + (Fields::Extension(last_alpha) * eval_wb_s_add_y
                + Fields::Extension(last_beta) * eval_wc_s_add_y)
            + (Fields::Extension(last_alpha) * eval_wb_s_mul
                + Fields::Extension(last_beta) * eval_wc_s_mul);

        for i in (1..circuit.layers.len()).rev() {
            let (add_i, mul_i) = LibraGKRLayeredCircuitTr::<F, E>::add_and_mul_mle(circuit, i - 1);

            // Gets w_i+1
            w_i_plus_one = output.layers[i - 1].clone();

            w_i_plus_one_iden = vec![F::from_canonical_u32(1); w_i_plus_one.len()];
            // let (sum_check_proof, sum_check_challenges) = perform_libra_sumcheck(&add_i, &mul_i, &w_i_plus_one, &w_i_plus_one_iden, &i_gz, &claimed_sum, &mut transcript)?;

            // PAUSE HERE
        }

        todo!()
    }

    fn verify(
        circuit: &LayeredCircuit,
        proofs: LibraProof<F, E>,
        input: Vec<F>,
    ) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
