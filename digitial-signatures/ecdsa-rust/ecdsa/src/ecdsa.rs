use ec_core::*;
use num_bigint::{BigUint, RandBigInt};
use sha256::digest;

pub struct ECDSA {
    elliptic_curve: EllipticCurve,
    a_gen: CurvePoint,
    q_order: BigUint,
}

#[derive(Debug)]
pub enum ECDSAErrors {
    BadArgument(String),
    OperationFailure(String),
}

impl ECDSA {
    // Generates: d, B where B = d A
    pub fn generate_key_pair(&self) -> Result<(BigUint, CurvePoint), ECDSAErrors> {
        let priv_key = self.generate_priv_key();
        let pub_key = self.generate_pub_key(&priv_key)?;
        Ok((priv_key, pub_key))
    }

    pub fn generate_priv_key(&self) -> BigUint {
        self.generate_random_positive_number_less_than(&self.q_order)
    }

    pub fn generate_pub_key(&self, priv_key: &BigUint) -> Result<CurvePoint, ECDSAErrors> {
        self.elliptic_curve
            .scalar_mul(&self.a_gen, priv_key)
            .map_err(|_| ECDSAErrors::OperationFailure("Error computing priv_key * a_gen".into()))
    }

    // (0, max)
    pub fn generate_random_positive_number_less_than(&self, max: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), max)
    }

    ///
    /// R = k A -> take `r = x` component
    /// s = (hash(message) + d * r) * k^(-1) mod q
    ///
    pub fn sign(
        &self,
        hash: &BigUint,
        priv_key: &BigUint,
        k_random: &BigUint,
    ) -> Result<(BigUint, BigUint), ECDSAErrors> {
        if *hash >= self.q_order {
            return Err(ECDSAErrors::BadArgument(
                "Hash is bigger than the order of the EC group".into(),
            ));
        }

        if *priv_key >= self.q_order {
            return Err(ECDSAErrors::BadArgument(
                "Private key is bigger than the order of the EC group".into(),
            ));
        }

        if *k_random >= self.q_order {
            return Err(ECDSAErrors::BadArgument(
                "Random number `k` is bigger than the order of the EC group".into(),
            ));
        }

        let r_point = self
            .elliptic_curve
            .scalar_mul(&self.a_gen, k_random)
            .map_err(|_| {
                ECDSAErrors::OperationFailure("Error computing k_random * a_gen".into())
            })?;

        if let CurvePoint::Coordinate(r, _) = r_point {
            let s = multiplicate(&r, priv_key, &self.q_order).map_err(|_| {
                ECDSAErrors::OperationFailure("Error multiplying r * priv_key".into())
            })?;

            let s = add(&s, hash, &self.q_order).map_err(|_| {
                ECDSAErrors::OperationFailure("Error adding hash + r * priv_key".into())
            })?;

            let k_inv = inverse_multiplicate_prime(k_random, &self.q_order)
                .map_err(|_| ECDSAErrors::OperationFailure("Error computing k_inv".into()))?;

            let s = multiplicate(&s, &k_inv, &self.q_order).map_err(|_| {
                ECDSAErrors::OperationFailure(
                    "Error computing (hash + r * priv_key) * k_inv".into(),
                )
            })?;

            return Ok((r, s));
        }

        Err(ECDSAErrors::OperationFailure(
            "Result k_random * a_gen is the identity".into(),
        ))
    }

    ///
    /// Verifies if a signature is valid for a particular message hash and public key.
    ///
    /// (s, r) = signature
    /// u1 = s^(-1) * hash(message) mod q
    /// u2 = s^(-1) * r mod q
    /// P = u1 A + u2 B mod q = (xp, yp)
    /// if r == xp then verified!
    ///
    pub fn verify(
        &self,
        hash: &BigUint,
        pub_key: &CurvePoint,
        signature: &(BigUint, BigUint),
    ) -> Result<bool, ECDSAErrors> {
        if *hash >= self.q_order {
            return Err(ECDSAErrors::BadArgument(
                "Hash value >= q (EC group order)".to_string(),
            ));
        }

        let (r, s) = signature;

        let s_inv = inverse_multiplicate_prime(s, &self.q_order)
            .map_err(|_| ECDSAErrors::OperationFailure("Error computing s_inv".into()))?;

        let u1 = multiplicate(&s_inv, hash, &self.q_order).map_err(|_| {
            ECDSAErrors::OperationFailure("Error multiplying s_inv and hash".into())
        })?;

        let u2 = multiplicate(&s_inv, r, &self.q_order)
            .map_err(|_| ECDSAErrors::OperationFailure("Error multiplying s_inv and r".into()))?;

        let u1a = self
            .elliptic_curve
            .scalar_mul(&self.a_gen, &u1)
            .map_err(|_| ECDSAErrors::OperationFailure("Error in u1 * a_gen".into()))?;

        let u2b = self
            .elliptic_curve
            .scalar_mul(pub_key, &u2)
            .map_err(|_| ECDSAErrors::OperationFailure("Error in u2 * pub_key".into()))?;

        let p = self
            .elliptic_curve
            .add(&u1a, &u2b)
            .map_err(|_| ECDSAErrors::OperationFailure("Error in u1a + u2b".into()))?;

        if let CurvePoint::Coordinate(xp, _) = p {
            return Ok(xp == *r);
        }

        Err(ECDSAErrors::OperationFailure(
            "Result is the identity".into(),
        ))
    }

    /// 0 < hash < max
    pub fn generate_hash_less_than(&self, message: &str, max: &BigUint) -> BigUint {
        let digest = digest(message);
        let hash_bytes = hex::decode(digest).expect("Could not convert hash to Vec<u8>");
        let hash = BigUint::from_bytes_be(&hash_bytes);
        let hash = hash.modpow(&BigUint::from(1u32), &(max - BigUint::from(1u32)));
        hash + BigUint::from(1u32)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let a_gen = CurvePoint::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &signature)
            .expect("Could not verify");

        assert!(verify_result, "Verification should success");
    }

    #[test]
    fn test_sign_verify_tempered_message() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let a_gen = CurvePoint::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::from(18u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");

        let message = "Bob -> 2 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &signature)
            .expect("Could not verify");

        assert!(
            !verify_result,
            "Verification should fail when message is tempered"
        );
    }

    #[test]
    fn test_sign_verify_tempered_signature() {
        let elliptic_curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let a_gen = CurvePoint::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        let q_order = BigUint::from(19u32);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::from(7u32);
        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::from(13u32);

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");

        let (r, s) = signature;

        let tempered_signature = (
            (r + BigUint::from(1u32)).modpow(&BigUint::from(1u32), &ecdsa.q_order),
            s,
        );

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &tempered_signature)
            .expect("Could not verify");

        assert!(
            !verify_result,
            "Verification should fail when signature is tempered"
        );
    }

    #[test]
    fn test_secp256_sign_verify() {
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
            .expect("could not convert p");

        let q_order = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
            .expect("could not convert n");

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
            .expect("could not convert gx");

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
            .expect("could not convert gy");

        let elliptic_curve = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let a_gen = CurvePoint::Coordinate(gx, gy);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::parse_bytes(
            b"483ADB7726A3C4655DA4FBFC0E1208A8F017B448A68554199C47D08FFB10E4B9",
            16,
        )
            .expect("Could not convert hex to private key");

        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::parse_bytes(
            b"19BE666EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B15E81798",
            16,
        )
            .expect("Could not convert hex to private key");

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &signature)
            .expect("Could not verify");

        assert!(verify_result, "Verification should have succeed");
    }

    #[test]
    fn test_secp256_sign_verify_tempered_message() {
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
            .expect("could not convert p");

        let q_order = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
            .expect("could not convert n");

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
            .expect("could not convert gx");

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
            .expect("could not convert gy");

        let elliptic_curve = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let a_gen = CurvePoint::Coordinate(gx, gy);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::parse_bytes(
            b"483ADB7726A3C4655DA4FBFC0E1208A8F017B448A68554199C47D08FFB10E4B9",
            16,
        )
            .expect("Could not convert hex to private key");

        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::parse_bytes(
            b"19BE666EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B15E81798",
            16,
        )
            .expect("Could not convert hex to private key");

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");

        let message = "Bob -> 2 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &signature)
            .expect("Could not verify");

        assert!(
            !verify_result,
            "Verification should have failed due to tempered message"
        );
    }

    #[test]
    fn test_secp256_sign_verify_tempered_signature() {
        let p = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
            .expect("could not convert p");

        let q_order = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
            .expect("could not convert n");

        let gx = BigUint::parse_bytes(
            b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
            .expect("could not convert gx");

        let gy = BigUint::parse_bytes(
            b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
            .expect("could not convert gy");

        let elliptic_curve = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p,
        };

        let a_gen = CurvePoint::Coordinate(gx, gy);

        let ecdsa = ECDSA {
            elliptic_curve,
            a_gen,
            q_order,
        };

        let priv_key = BigUint::parse_bytes(
            b"483ADB7726A3C4655DA4FBFC0E1208A8F017B448A68554199C47D08FFB10E4B9",
            16,
        )
            .expect("Could not convert hex to private key");

        let pub_key = ecdsa
            .generate_pub_key(&priv_key)
            .expect("Could not compute PubKey");

        let k_random = BigUint::parse_bytes(
            b"19BE666EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B15E81798",
            16,
        )
            .expect("Could not convert hex to private key");

        let message = "Bob -> 1 BTC -> Alice";
        let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

        let signature = ecdsa
            .sign(&hash, &priv_key, &k_random)
            .expect("Could not sign");
        let (r, s) = signature;

        let tempered_signature = (
            (r + BigUint::from(1u32)).modpow(&BigUint::from(1u32), &ecdsa.q_order),
            s,
        );

        let verify_result = ecdsa
            .verify(&hash, &pub_key, &tempered_signature)
            .expect("Could not verify");

        assert!(
            !verify_result,
            "Verification should have failed due to tempered signature"
        );
    }
}
