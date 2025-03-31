//! This file contains the implementation of the Transcript struct and interface.
use ark_ff::PrimeField;
use fiat_shamir::{FiatShamirTranscript, TranscriptInterface};

pub trait TranscriptProtocol<F: PrimeField> {
    /// Append a domain separator for sumcheck proof of with `num_vars` variables and degree `m` of combine function.
    fn sumcheck_proof_domain_sep(&mut self, num_vars: u64, m: u64);

    /// Append a `scalar` with the given `label`.
    fn append_scalar(&mut self, label: &str, scalar: &F);

    /// Append multiple `scalars` with the given `label`.
    fn append_scalars(&mut self, label: &str, scalars: &[F]);

    /// Compute a `label`ed challenge variable.
    fn challenge_scalar(&mut self, label: &'static [u8]) -> F;

    /// Compute a `label`ed vector of challenges.
    fn challenge_vector(&mut self, label: &'static [u8], len: usize) -> Vec<F>;
}

impl<F: PrimeField> TranscriptProtocol<F> for FiatShamirTranscript {
    fn sumcheck_proof_domain_sep(&mut self, num_vars: u64, m: u64) {
        self.append_with_label("domain-separator", b"sumcheck v1".to_vec());
        self.append_with_label("n", num_vars.to_be_bytes().to_vec());
        self.append_with_label("m", m.to_be_bytes().to_vec());
    }

    fn append_scalar(&mut self, label: &str, scalar: &F) {
        let mut buf = vec![];
        scalar.serialize_compressed(&mut buf).unwrap();
        self.append_with_label(label, buf);
    }

    fn append_scalars(&mut self, label: &str, scalars: &[F]) {
        self.append_with_label(label, b"begin_append_vector".to_vec());
        for item in scalars.iter() {
            <Self as TranscriptProtocol<F>>::append_scalar(self, label, item);
        }
        self.append_with_label(label, b"end_append_vector".to_vec());
    }

    fn challenge_scalar(&mut self, _label: &'static [u8]) -> F {
        self.sample_as_field_element()
    }

    fn challenge_vector(&mut self, _label: &'static [u8], len: usize) -> Vec<F> {
        self.sample_n_as_field_elements(len)
    }
}
