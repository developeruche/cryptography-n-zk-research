use crate::primitives::{XmssPublicData, XmssSignature};
use m_tree::{DefaultMerkleTree, Digest, Sha256};
use winternitz_ots::sha256_plus::WotsPrivateKey;

pub struct XmssScheme {
    /// This is the height of the tree
    pub h: u64,
    /// This is the max counter attempts
    pub max_attempts: u32,
    pub private_keys: Vec<WotsPrivateKey>,
    pub public_data: XmssPublicData,
    pub current_index: u64,
    pub pubkey_tree: DefaultMerkleTree,
}

impl XmssScheme {
    pub fn init_scheme(max_attempts: u32, max_signatures: u64) -> Self {
        // makes sure max_signature is a power of 2
        assert!(max_signatures.is_power_of_two());
        let h = max_signatures.trailing_zeros() as u64;

        let mut private_keys = Vec::with_capacity(max_signatures as usize);
        let mut public_keys = Vec::with_capacity(max_signatures as usize);
        let mut public_data = XmssPublicData {
            max_signatures,
            root: [0; 32],
        };

        for _ in 0..max_signatures {
            let private_key = WotsPrivateKey::new();
            let public_key = private_key.to_public();

            private_keys.push(private_key);
            public_keys.push(public_key.hash());
        }

        let pubkey_tree = DefaultMerkleTree::from_leaves(&public_keys);
        public_data.root = pubkey_tree.root().expect("Failed to get root");

        Self {
            h,
            max_attempts,
            private_keys,
            public_data,
            current_index: 0,
            pubkey_tree,
        }
    }

    pub fn sign(&mut self, message: &[u8]) -> XmssSignature {
        assert!(
            self.current_index < self.public_data.max_signatures,
            "XMSS Error: No more OTP keys available. Signatures exhausted."
        );

        let index = self.current_index as usize;
        let (wots_signature, counter) =
            self.private_keys[index].sign_optimized(message, self.max_attempts);
        let auth_path = self.pubkey_tree.proof(index).expect("Failed to get proof");

        let signature = XmssSignature {
            auth_path,
            wots_signature,
            public_key: self.private_keys[index].to_public(),
            counter,
            index: self.current_index,
        };

        self.current_index += 1;
        signature
    }

    pub fn verify(&self, message: &[u8], signature: &XmssSignature) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(message);
        hasher.update(&signature.counter.to_be_bytes());
        let message_hash = hasher.finalize().into();

        if !signature.auth_path.verify(
            &self.public_data.root,
            signature.index as usize,
            &signature.public_key.hash(),
            self.public_data.max_signatures as usize,
        ) {
            return false;
        }

        if !signature
            .public_key
            .verify(&message_hash, &signature.wots_signature)
        {
            return false;
        }

        true
    }

    pub fn remaining_signatures(&self) -> u64 {
        self.public_data.max_signatures - self.current_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // Test initializing with 4 signatures (height 2)
        let xmss = XmssScheme::init_scheme(255, 4);
        assert_eq!(xmss.h, 2);
        assert_eq!(xmss.private_keys.len(), 4);
        assert_eq!(xmss.current_index, 0);
        assert_eq!(xmss.remaining_signatures(), 4);
        assert_ne!(xmss.public_data.root, [0u8; 32]);
    }

    #[test]
    #[should_panic]
    fn test_init_invalid_signatures() {
        // Should panic because 3 is not a power of 2
        XmssScheme::init_scheme(255, 3);
    }

    #[test]
    fn test_sign_verify_success() {
        let mut xmss = XmssScheme::init_scheme(255, 4);
        let message = b"Hello XMSS";

        let signature = xmss.sign(message);
        assert!(
            xmss.verify(message, &signature),
            "Signature validation failed"
        );

        // Check that index advanced
        assert_eq!(xmss.current_index, 1);
        assert_eq!(xmss.remaining_signatures(), 3);
    }

    #[test]
    fn test_verify_failures() {
        let mut xmss = XmssScheme::init_scheme(255, 4);
        let message = b"Valid Message";
        let signature = xmss.sign(message);

        // Wrong Message
        assert!(
            !xmss.verify(b"Wrong Message", &signature),
            "Should fail for wrong message"
        );

        // Wrong Counter (if we could modify it, but we can construct a new signature struct)
        let wrong_time_sig = XmssSignature {
            auth_path: signature.auth_path.clone(),
            wots_signature: signature.wots_signature.clone(),
            public_key: signature.public_key.clone(),
            counter: signature.counter + 1, // Invalid counter for this internal WOTS sig, this would change this message this is been signed
            index: signature.index,
        };
        assert!(
            !xmss.verify(message, &wrong_time_sig),
            "Should fail for wrong counter"
        );

        // Verification with wrong instance (though root is what matters)
        let xmss2 = XmssScheme::init_scheme(255, 4);
        assert!(
            !xmss2.verify(message, &signature),
            "Should fail against different public root"
        );
    }

    #[test]
    fn test_signature_capacity_exhaustion() {
        let max_sigs = 2;
        let mut xmss = XmssScheme::init_scheme(255, max_sigs);

        // Sign 1
        xmss.sign(b"Msg 1");
        assert_eq!(xmss.remaining_signatures(), 1);

        // Sign 2
        xmss.sign(b"Msg 2");
        assert_eq!(xmss.remaining_signatures(), 0);
    }

    #[test]
    #[should_panic(expected = "Signatures exhausted")]
    fn test_signing_too_many() {
        let max_sigs = 2;
        let mut xmss = XmssScheme::init_scheme(255, max_sigs);
        xmss.sign(b"1");
        xmss.sign(b"2");
        // This third call should panic
        xmss.sign(b"3");
    }
}
