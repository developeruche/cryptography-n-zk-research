pub mod interface;
use sha3::{Digest, Keccak256};
use interface::TranscriptInterface;

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
}