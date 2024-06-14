use ark_ff::Field;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Witness<F: Field> {
    /// The public input to the circuit
    pub public_input: Vec<F>,
    /// The auxiliary input to the circuit
    pub auxiliary_input: Vec<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct R1CS<F> {
    /// This is the C matrix
    c: Vec<Vec<F>>,
    /// This is the A matrix
    a: Vec<Vec<F>>,
    /// This is the B matrix
    b: Vec<Vec<F>>,
}





impl<F: Field> Witness<F> {
    pub fn render(&self) -> Vec<F> {
        let mut ren = self.public_input.clone();
        ren.extend(self.auxiliary_input.clone());
        ren
    }
}