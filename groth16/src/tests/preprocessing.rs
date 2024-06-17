use ark_test_curves::bls12_381::Fr;
use crate::primitives::R1CS;


#[test]
fn test_to_qap_poly_coefficients() {
    let r1cs = R1CS::<Fr> {
        a: vec![vec![Fr::from(2u32), Fr::from(1u32)], vec![Fr::from(2u32), Fr::from(5u32)]],
        b: vec![vec![Fr::from(2u32), Fr::from(2u32)], vec![Fr::from(2u32), Fr::from(2u32)]],
        c: vec![vec![Fr::from(2u32), Fr::from(2u32)], vec![Fr::from(2u32), Fr::from(2u32)]],
    };
    
}