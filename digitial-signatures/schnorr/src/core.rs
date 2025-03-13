pub use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve,
    },
    unsigned_integer::element::U256,
};
use lambdaworks_math::{
    elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint,
    traits::{AsBytes, ByteConversion},
};
use sha256::digest;

pub struct KeyPair {
    pub private_key: U256,
    pub public_key: ShortWeierstrassProjectivePoint<BLS12381Curve>,
}

pub struct Signature {
    pub s: U256,
    pub e: U256,
}

impl KeyPair {
    /// This function is used to generate a new key pair
    pub fn new(private_key: U256) -> Self {
        let generator = BLS12381Curve::generator();
        let public_key = generator.operate_with_self(private_key);

        KeyPair {
            private_key,
            public_key,
        }
    }
}

impl Signature {
    pub fn new(s: U256, e: U256) -> Self {
        Signature { s, e }
    }

    /// This function is used for verifying a signature over a message
    pub fn verify(
        &self,
        message: String,
        public_key: ShortWeierstrassProjectivePoint<BLS12381Curve>,
    ) -> anyhow::Result<bool, anyhow::Error> {
        let generator = BLS12381Curve::generator();
        let g_pow_s = generator.operate_with_self(self.s);
        let public_key_pow_e = public_key.operate_with_self(self.e);

        // g^s * public_key^e
        let r_v = g_pow_s.operate_with(&public_key_pow_e);

        let mut e_preimage = r_v.as_bytes();
        e_preimage.extend(message.as_bytes().to_vec());
        let e_raw = digest(e_preimage);
        let e = U256::from_bytes_be(&e_raw.as_bytes()).expect("Failed to convert bytes to U256");

        Ok(e == self.e)
    }
}
