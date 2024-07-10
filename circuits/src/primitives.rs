#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GateType {
    /// This represents an addtion gate
    Add,
    /// This represents a multipication gate
    Mul,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Gate {
    /// This represents the gate-type
    pub g_type: GateType,
    /// This represents the inputs to this gate (this input to the gate are two finite field element)
    pub inputs: [usize; 2],
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Constraints {
    pub a_s: Vec<usize>,
    pub b_s: Vec<usize>,
    pub c_s: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct CircuitLayer {
    /// This circuit layer is just a row of gates
    pub layer: Vec<Gate>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Circuit {
    /// The circuit is a vector of layers
    pub layers: Vec<CircuitLayer>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct CircuitEvaluation<F> {
    /// This is the curcuit evaluation on every layer
    pub layers: Vec<Vec<F>>,
}

impl Gate {
    pub fn new(g_type: GateType, inputs: [usize; 2]) -> Self {
        Gate { g_type, inputs }
    }
}

impl CircuitLayer {
    pub fn new(layer: Vec<Gate>) -> Self {
        CircuitLayer { layer }
    }
}

impl Circuit {
    pub fn new(layers: Vec<CircuitLayer>) -> Self {
        Circuit { layers }
    }
}

impl<F> CircuitEvaluation<F> {
    pub fn new(layers: Vec<Vec<F>>) -> Self {
        CircuitEvaluation { layers }
    }
}
