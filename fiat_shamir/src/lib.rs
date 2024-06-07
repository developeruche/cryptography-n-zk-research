pub mod interface;
use interface::TranscriptInterface;
use sha3::{Digest, Keccak256};

#[derive(Clone, Default, Debug)]
pub struct FiatShamirTranscript {
    hasher: Keccak256,
}

impl FiatShamirTranscript {
    pub fn new(msg: Vec<u8>) -> Self {
        let mut response = Self {
            hasher: Keccak256::new(),
        };
        response.append(msg);
        response
    }
}

impl TranscriptInterface for FiatShamirTranscript {
    fn append(&mut self, msg: Vec<u8>) {
        self.hasher.update(&msg);
    }

    fn sample(&mut self) -> [u8; 32] {
        let response = self.hasher.finalize_reset();
        self.hasher.update(&response);
        response.into()
    }

    fn sample_n(&mut self, n: usize) -> Vec<[u8; 32]> {
        let mut response = Vec::new();
        for _ in 0..n {
            response.push(self.sample());
        }
        response
    }

    fn sample_n_as_field_elements<F: ark_ff::prelude::PrimeField>(&mut self, n: usize) -> Vec<F> {
        let mut response = Vec::new();
        for _ in 0..n {
            response.push(F::from_be_bytes_mod_order(&self.sample()));
        }
        response
    }
}
