// //! An implementation of the Libra protocol.
// use circuits::{interface::LibraGKRLayeredCircuitTr, layered_circuit::{primitives::Evaluation, LayeredCircuit}};
// use p3_field::{ExtensionField, Field, PrimeField32};
// use poly::{mle::MultilinearPoly, Fields, MultilinearExtension};
// use primitives::LibraProof;
// use transcript::Transcript;

// use crate::{utils::generate_igz, LinearTimeSumCheck, LinearTimeSumCheckTr};
// pub mod primitives;

// /// Interface for the Libra protocol.
// pub trait LibraTr<F: Field + PrimeField32, E: ExtensionField<F>> {
//     fn prove(circuit: &LayeredCircuit, output: Evaluation<F>) -> Result<LibraProof<F, E>, anyhow::Error>;
//     fn verify(
//         circuit: &LayeredCircuit,
//         proofs: LibraProof<F, E>,
//         input: Vec<F>,
//     ) -> Result<bool, anyhow::Error>;
// }

// /// A struct representing the Libra protocol.
// pub struct Libra<F: Field, E: ExtensionField<F>> {
//     _marker: std::marker::PhantomData<(F, E)>,
// }

// impl<F: Field + PrimeField32, E: ExtensionField<F>> LibraTr<F, E> for Libra<F, E> {
//     fn prove(circuit: &LayeredCircuit, output: Evaluation<F>) -> Result<LibraProof<F, E>, anyhow::Error> {
//         // Initialize prover transcript
//         let mut transcript = Transcript::<F, E>::init();
//         let mut sumcheck_proofs = vec![];
//         let mut wbs = vec![];
//         let mut wcs = vec![];

//         // Get the output vector
//         let mut output_evals: Vec<Fields<F, E>> = output.layers[circuit.layers.len()]
//             .iter()
//             .map(|val| Fields::<F, E>::Base(*val))
//             .collect();

//         if output_evals.len() == 1 {
//             output_evals.push(Fields::Base(F::zero()));
//         }

//         // Build the output polynomial
//         let output_mle = MultilinearPoly::new_from_vec(
//             (output_evals.len() as f64).log2() as usize,
//             output_evals,
//         );

//         // Adds the output to the transcript
//         transcript.observe_base_element(&output.layers[circuit.layers.len()]);

//         // Gets the addi and muli for the output layer
//         let (add_i, mul_i) =
//             LibraGKRLayeredCircuitTr::<F, E>::add_and_mul_mle(circuit, circuit.layers.len() - 1);

//         // Gets w_i+1
//         let mut w_i_plus_one_poly = output.layers[circuit.layers.len() - 1];

//         // Sample random challenge for the first round
//         let g = transcript.sample_n_challenges(output_mle.num_vars());
//         let mut i_gz = generate_igz(&g);

//         // at this point we've got all we need to run the 3 sum check protocols
//         // one
//         let layer_1_mul_i_proof = LinearTimeSumCheck::sum_check(&mul_i, &w_i_plus_one_poly, &i_gz, &mut transcript)?;

//         todo!()
//     }

//     fn verify(
//         circuit: &LayeredCircuit,
//         proofs: LibraProof<F, E>,
//         input: Vec<F>,
//     ) -> Result<bool, anyhow::Error> {
//         todo!()
//     }
// }
