use ark_ff::PrimeField;

/// This is a generic interface for the fiat shamir transcript.
pub trait TranscriptInterface {
    /// This is used to append a message to the transcript.
    fn append(&mut self, msg: Vec<u8>);
    /// This is used to appended a message to the transcript alongside a label
    fn append_with_label(&mut self, label: &str, msg: Vec<u8>) {
        let mut label_bytes = label.as_bytes().to_vec();
        label_bytes.append(&mut msg.clone());
        self.append(label_bytes);
    }
    /// This is used to get the challenge from the transcript.
    fn sample(&mut self) -> [u8; 32];
    /// This is used to sample n challenges from the transcript.
    fn sample_n(&mut self, n: usize) -> Vec<[u8; 32]>;
    /// This is used to sample n challenges from the transcript as  field elements.
    fn sample_n_as_field_elements<F: PrimeField>(&mut self, n: usize) -> Vec<F>;
    /// This is used to sample a challenge from the transcript as a field element.
    fn sample_as_field_element<F: PrimeField>(&mut self) -> F;
}
