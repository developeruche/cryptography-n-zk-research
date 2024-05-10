use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::{short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve}, unsigned_integer::{element::U256}};
use lambdaworks_math::traits::{AsBytes, ByteConversion};


pub struct KeyPair {
    pub private_key: U256,
    pub public_key: U256,
}



impl KeyPair {
    /// This function is used to generate a new key pair
    pub fn new(private_key: U256) -> Self {
        let generator = BLS12381Curve::generator();
        let public_key = generator.operate_with_self(private_key);

        KeyPair {
            private_key,
            public_key: U256::from_bytes_be(&public_key.as_bytes()).expect("Failed to convert bytes to U256"),
        }
    }
}
