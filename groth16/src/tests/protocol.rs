use crate::{
    interfaces::TrustedSetupInterface,
    primitives::{ToxicWaste, TrustedSetup, R1CS},
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
        TrustedSetup::run_trusted_setup(&toxic_waste, qap_poly, 3);
}
