use crate::{
    interfaces::{ProtocolInterface, R1CSProcessingInterface, TrustedSetupInterface, PreProcessorInterface}, preprocessing::PreProcessor, primitives::{ProofRands, ToxicWaste, TrustedSetup, Witness, R1CS}, protocol::Groth16Protocol
};
use ark_test_curves::bls12_381::Fr;

#[test]
fn test_valid_protocol() {
    let r1cs = R1CS::<Fr> {
        a: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(3u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
        b: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(5u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
        c: vec![
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(-3),
                Fr::from(1u32),
                Fr::from(1u32),
                Fr::from(2u32),
                Fr::from(0u32),
                Fr::from(-1),
            ],
        ],
    };

    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();
    let qap_poly = qap_poly_coefficients.into_poly_rep();
    let toxic_waste = ToxicWaste::random();
    let trusted_setup =
        TrustedSetup::<ark_test_curves::bls12_381::Bls12_381>::run_trusted_setup(&toxic_waste, &qap_poly, 3);
    let proof_rands = ProofRands::<Fr>::random();
    let out = Fr::from(24u32) + Fr::from(20u32) - Fr::from(2u32) - Fr::from(4u32) + Fr::from(3u32);
    let witness = Witness::new(
        vec![Fr::from(1u32), out],
        vec![
            Fr::from(2u32),
            Fr::from(2u32),
            Fr::from(12u32),
            Fr::from(24u32),
        ],
    );
    let preprocessor = PreProcessor::new(r1cs, witness.clone());
    let qap = preprocessor.preprocess();
    let groth16_proof = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::generate_proof(proof_rands, &trusted_setup, &qap, &witness);
    let is_valid = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::verify_proof(&groth16_proof, &trusted_setup, &witness.public_input);
    
    assert!(is_valid);
}
