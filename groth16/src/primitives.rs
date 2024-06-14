use ark_ff::Field;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Witness<F: Field> {
    // The public input to the circuit
    pub public_input: Vec<F>,
    // The auxiliary input to the circuit
    pub auxiliary_input: Vec<F>,
}