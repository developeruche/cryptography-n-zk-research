use rand::Rng;

pub struct PublicKey {}
pub struct SecretKey {}
pub struct Signature {}

impl PublicKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl SecretKey {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn get_prepared_interval(&self) -> std::ops::Range<u64> {
        todo!()
    }
}

impl Signature {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

pub struct SimpleLeanSig {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
}

impl SimpleLeanSig {
    // Placeholder lifetime
    pub const LIFETIME: u64 = 0;

    pub fn new(public_key: PublicKey, secret_key: SecretKey) -> Self {
        Self {
            public_key,
            secret_key,
        }
    }

    pub fn key_gen(
        _rng: &mut impl Rng,
        _activation_epoch: usize,
        _num_active_epochs: usize,
    ) -> (PublicKey, SecretKey) {
        // Todo: Implement actual key generation
        (PublicKey {}, SecretKey {})
    }

    pub fn sign(
        _secret_key: &SecretKey,
        _epoch: u32,
        _message: &[u8; 32],
    ) -> Result<Signature, anyhow::Error> {
        // Todo: Implement actual signing
        Ok(Signature {})
    }

    pub fn verify(
        _public_key: &PublicKey,
        _epoch: u32,
        _message: &[u8; 32],
        _signature: &Signature,
    ) -> bool {
        // Todo: Implement actual verification
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // leansig is in dev-dependencies, so we can use it here
    use leansig::{
        serialization::Serializable,
        signature::{SignatureScheme, SignatureSchemeSecretKey},
    };

    pub type LeanSignatureScheme = leansig::signature::generalized_xmss::instantiations_poseidon_top_level::lifetime_2_to_the_32::hashing_optimized::SIGTopLevelTargetSumLifetime32Dim64Base8;
    pub type LeanSigSignature = <LeanSignatureScheme as SignatureScheme>::Signature;
    pub type LeanSigPublicKey = <LeanSignatureScheme as SignatureScheme>::PublicKey;
    pub type LeanSigSecretKey = <LeanSignatureScheme as SignatureScheme>::SecretKey;

    const MESSAGE: &[u8; 32] = b"hello world, how are you doing?!";

    fn run_lean_sig() -> (LeanSigPublicKey, LeanSigSecretKey, LeanSigSignature) {
        let lifetime = 32;
        let mut rng = rand::rng();
        let (lean_pk, lean_sk) = LeanSignatureScheme::key_gen(&mut rng, 0, lifetime as usize);

        let prepared_interval = lean_sk.get_prepared_interval();
        let epoch = rng.random_range(prepared_interval.start as u32..prepared_interval.end as u32);

        let lean_sig = LeanSignatureScheme::sign(&lean_sk, epoch, MESSAGE).expect("Signing failed");

        assert!(LeanSignatureScheme::verify(
            &lean_pk, epoch, MESSAGE, &lean_sig
        ));

        println!("Lean Public Key: {:?}", lean_pk.to_bytes());
        println!("Lean Secret Key: {:?}", lean_sk.to_bytes());
        println!("Lean Signature: {:?}", lean_sig.to_bytes());

        (lean_pk, lean_sk, lean_sig)
    }

    fn run_simple_lean_sig() -> (PublicKey, SecretKey, Signature) {
        let lifetime = 32;
        let mut rng = rand::rng();

        let (simple_lean_pk, simple_lean_sk) =
            SimpleLeanSig::key_gen(&mut rng, 0, lifetime as usize);

        println!("Lean Public Key: {:?}", simple_lean_pk.to_bytes());
        println!("Lean Secret Key: {:?}", simple_lean_sk.to_bytes());

        let prepared_interval = simple_lean_sk.get_prepared_interval();
        let epoch = rng.random_range(prepared_interval.start as u32..prepared_interval.end as u32);
        println!("Epoch: {}", epoch);

        let lean_sig =
            SimpleLeanSig::sign(&simple_lean_sk, epoch, MESSAGE).expect("Signing failed");
        println!("Signature: {:?}", lean_sig.to_bytes());

        // Verify using the formal scheme
        assert!(SimpleLeanSig::verify(
            &simple_lean_pk,
            epoch,
            MESSAGE,
            &lean_sig
        ));

        // Return the keys wrapped in our new SimpleLeanSig types
        (simple_lean_pk, simple_lean_sk, lean_sig)
    }

    #[test]
    fn lean_sig_n_simple_lean_sig_compact_test() {
        let (_public_key, _secret_key, _signature) = run_simple_lean_sig();
        let (_lean_pk, _lean_sk, _lean_sig) = run_lean_sig();
    }
}
