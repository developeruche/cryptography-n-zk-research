use ark_ff::PrimeField;
use polynomial::{multilinear::Multilinear, univariant::UnivariantPolynomial};
use sum_check::data_structure::SumCheckProof;

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

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct W<F: PrimeField> {
    /// This is the addition multilinear extension
    add_i: Multilinear<F>,
    /// This is the multiplication multilinear extension
    mul_i: Multilinear<F>,
    /// This is the w_b equation
    w_b: Multilinear<F>,
    /// This is the w_c equation
    w_c: Multilinear<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct GKRProof<F: PrimeField> {
    /// This is the output of the Circuit evaluation
    pub output: Vec<F>,
    /// This is the list of sum check proofs gotten during this protocol
    pub sum_check_proofs: Vec<SumCheckProof<F>>,
    /// This is the list of q polynomials
    pub q_polynomials: Vec<UnivariantPolynomial<F>>,
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

impl<F: PrimeField> W<F> {
    pub fn new(
        add_i: Multilinear<F>,
        mul_i: Multilinear<F>,
        w_b: Multilinear<F>,
        w_c: Multilinear<F>,
    ) -> Self {
        W {
            add_i,
            mul_i,
            w_b,
            w_c,
        }
    }
}
