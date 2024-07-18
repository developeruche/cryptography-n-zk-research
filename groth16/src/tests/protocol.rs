use crate::{
    interfaces::{
        PreProcessorInterface, ProtocolInterface, R1CSProcessingInterface, TrustedSetupInterface,
    },
    preprocessing::PreProcessor,
    primitives::{ProofRands, ToxicWaste, TrustedSetup},
    protocol::Groth16Protocol,
};
use ark_test_curves::bls12_381::Fr;
use circuits::{
    interfaces::ExtractConstraintsInterface,
    primitives::{Circuit, CircuitLayer, Gate, GateType, Witness, R1CS},
};
use polynomial::interface::PolynomialInterface;

#[test]
fn test_valid_protocol() {
    let r1cs = R1CS::<Fr> {
        a: vec![
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
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
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
        b: vec![
            vec![
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
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
                Fr::from(1u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
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
                Fr::from(0u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(1u32),
                Fr::from(0u32),
            ],
        ],
        c: vec![
            vec![
                Fr::from(-2),
                Fr::from(3u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(-2),
                Fr::from(0u32),
                Fr::from(3u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(-2),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(3u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(-2),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(3u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
                Fr::from(0u32),
            ],
        ],
    };

    let witness = Witness::new(
        vec![Fr::from(1u32)],
        vec![
            Fr::from(1u32),
            Fr::from(2u32),
            Fr::from(1u32),
            Fr::from(2u32),
        ],
    );

    let r1cs_check = r1cs.check(witness.render());
    assert!(r1cs_check, "this is the R1CS check");

    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();
    let qap_poly = qap_poly_coefficients.into_poly_rep();

    let preprocessor = PreProcessor::new(r1cs, witness.clone());
    let qap = preprocessor.preprocess();

    let check = qap.qap_check();
    assert_eq!(check, true);

    let toxic_waste = ToxicWaste::new(
        Fr::from(2u32),
        Fr::from(3u32),
        Fr::from(5u32),
        Fr::from(6u32),
        Fr::from(4u32),
    );

    let trusted_setup = TrustedSetup::<ark_test_curves::bls12_381::Bls12_381>::run_trusted_setup(
        &toxic_waste,
        &qap_poly,
        qap.ax.degree(),
    );

    let proof_rands = ProofRands::<Fr>::new(Fr::from(3u32), Fr::from(5u32));

    let groth16_proof = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::generate_proof(
        proof_rands,
        &trusted_setup,
        &qap,
        &witness,
    );

    let is_valid = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::verify_proof(
        &groth16_proof,
        &trusted_setup,
        &witness.public_input,
    );

    assert!(is_valid);
}

#[test]
fn test_valid_protocol_2() {
    let r1cs = R1CS::<Fr> {
        a: vec![vec![
            Fr::from(0u32),
            Fr::from(0u32),
            Fr::from(1u32),
            Fr::from(0u32),
        ]],
        b: vec![vec![
            Fr::from(0u32),
            Fr::from(0u32),
            Fr::from(0u32),
            Fr::from(1u32),
        ]],
        c: vec![vec![
            Fr::from(0u32),
            Fr::from(1u32),
            Fr::from(0u32),
            Fr::from(0u32),
        ]],
    };

    let witness = Witness::new(
        vec![Fr::from(1u32)],
        vec![Fr::from(4223u32), Fr::from(41u32), Fr::from(103u32)],
    );

    let r1cs_check = r1cs.check(witness.render());
    assert!(r1cs_check, "this is the R1CS check");

    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();
    let qap_poly = qap_poly_coefficients.into_poly_rep();

    let preprocessor = PreProcessor::new(r1cs, witness.clone());
    let qap = preprocessor.preprocess();

    let check = qap.qap_check();
    assert_eq!(check, true);

    let toxic_waste = ToxicWaste::new(
        Fr::from(2u32),
        Fr::from(3u32),
        Fr::from(5u32),
        Fr::from(6u32),
        Fr::from(4u32),
    );

    let trusted_setup = TrustedSetup::<ark_test_curves::bls12_381::Bls12_381>::run_trusted_setup(
        &toxic_waste,
        &qap_poly,
        qap.ax.degree(),
    );

    let proof_rands = ProofRands::<Fr>::new(Fr::from(3u32), Fr::from(5u32));

    let groth16_proof = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::generate_proof(
        proof_rands,
        &trusted_setup,
        &qap,
        &witness,
    );

    let is_valid = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::verify_proof(
        &groth16_proof,
        &trusted_setup,
        &witness.public_input,
    );

    assert!(is_valid);
}

// running groth16 on this circuit
//      100(*)    - layer 0
//     /     \
//   5(+)_0   20(*)_1 - layer 1
//   / \    /  \
//  2   3   4   5
#[test]
fn test_valid_protocol_on_arithmetic_circuit() {
    let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
    let layer_1 = CircuitLayer::new(vec![
        Gate::new(GateType::Mul, [0, 1]),
        Gate::new(GateType::Add, [2, 3]),
    ]);

    let circuit = Circuit::new(vec![layer_0, layer_1]);
    let constraints = circuit.extract_constraints();

    let r1cs = constraints.to_r1cs_vec::<Fr>();
    let witness = Witness::new(
        vec![Fr::from(1u32)],
        vec![
            Fr::from(15u32),
            Fr::from(6u32),
            Fr::from(9u32),
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(4u32),
            Fr::from(5u32),
        ],
    );
    let r1cs_check = r1cs.check(witness.render());
    assert!(r1cs_check, "this is the R1CS check");

    let qap_poly_coefficients = r1cs.to_qap_poly_coefficients();
    let qap_poly = qap_poly_coefficients.into_poly_rep();

    let preprocessor = PreProcessor::new(r1cs, witness.clone());
    let qap = preprocessor.preprocess();

    // let check = qap.qap_check();
    // assert_eq!(check, true);

    // let toxic_waste = ToxicWaste::new(
    //     Fr::from(2u32),
    //     Fr::from(3u32),
    //     Fr::from(5u32),
    //     Fr::from(6u32),
    //     Fr::from(4u32),
    // );

    // let trusted_setup = TrustedSetup::<ark_test_curves::bls12_381::Bls12_381>::run_trusted_setup(
    //     &toxic_waste,
    //     &qap_poly,
    //     qap.ax.degree(),
    // );

    // let proof_rands = ProofRands::<Fr>::new(Fr::from(3u32), Fr::from(5u32));

    // let groth16_proof = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::generate_proof(
    //     proof_rands,
    //     &trusted_setup,
    //     &qap,
    //     &witness,
    // );

    // let is_valid = Groth16Protocol::<ark_test_curves::bls12_381::Bls12_381>::verify_proof(
    //     &groth16_proof,
    //     &trusted_setup,
    //     &witness.public_input,
    // );

    // assert!(is_valid);
}
