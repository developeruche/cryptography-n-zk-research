/// This is a generic interface for the fiat shamir transcript.
pub trait TranscriptInterface {
    /// This is used to append a message to the transcript.
    fn append(&mut self, msg: Vec<u8>);
    /// This is used to get the challenge from the transcript.
    fn sample(&mut self) -> [u8; 32];
    /// This is used to sample n challenges from the transcript.
    fn sample_n(&mut self, n: usize) -> Vec<[u8; 32]>;
}
