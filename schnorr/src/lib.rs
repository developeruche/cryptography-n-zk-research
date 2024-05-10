use core::Signature;

use sha256::digest;

use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::{short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve}, traits::{AsBytes, ByteConversion}, unsigned_integer::element::U256};
use rand::prelude::*;

pub mod core;


/// This function is used to sign a message over a generic elliptic curve and generic hash function.
pub fn sign(private_key: U256, message: String) -> anyhow::Result<Signature, anyhow::Error> {
    // Generate out very secured random scalar
    let mut randomness_engine = rand::thread_rng();
    let raw_k = digest(randomness_engine.gen_range(0..100).to_string());
    let k = U256::from_bytes_be(&raw_k.as_bytes()).expect("Failed to convert bytes to U256");
    // r = G.pow(k)
    let r = BLS12381Curve::generator().operate_with_self(k);

    let mut e_preimage = r.as_bytes();
    // (r || message)
    e_preimage.extend(message.as_bytes().to_vec());
    // H (r || message)
    let e_raw = digest(e_preimage);
    let e = U256::from_bytes_be(&e_raw.as_bytes()).expect("Failed to convert bytes to U256");

    // s = k - e * private_key
    let s = k - (e * private_key);

    Ok(
        Signature::new(s, e)
    )
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::core::KeyPair;

    use super::*;

    #[test]
    fn it_works() {
        let key_pair = KeyPair::new(U256::from(123456789u128));
        let message = "Hello, world!".to_string();
        let signature = sign(key_pair.private_key, message.clone()).unwrap();
        assert!(signature.verify(message, key_pair.public_key).unwrap());
    }
}
