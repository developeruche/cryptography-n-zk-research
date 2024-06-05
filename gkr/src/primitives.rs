use ark_ff::PrimeField;



#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub enum GateType {
    /// This represents an addtion gate
    Add, 
    /// This represents a multipication gate
    Mul
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Gate<F: PrimeField> {
    /// This represents the gate-type
    pub g_type: GateType,
    /// This represents the inputs to this gate (this input to the gate are two finite field element)
    pub inputs: [F; 2]
}


#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct CircuitLayer<F: PrimeField> {
    /// This circuit layer is just a row of gates
    pub layer: Vec<Gate<F>>
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Circuit<F: PrimeField> {
    /// The circuit is a vector of layers
    pub layers: Vec<CircuitLayer<F>>
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct CircuitEvaluation<F: PrimeField> {
    /// This is the curcuit evaluation on every layer
    pub layers: Vec<Vec<F>>,
}