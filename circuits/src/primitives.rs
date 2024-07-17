use crate::utils::check_init;
use ark_ff::PrimeField;
use std::collections::HashMap;

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
pub struct Constraint {
    // a, b, c; where c = a.b;
    pub a: Vec<usize>,
    pub b: Vec<usize>,
    pub c: Vec<usize>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstraintsWithLabelSize {
    pub constraints: Vec<Constraint>,
    pub label_size: usize,
}

pub struct ConstraintRaw {
    pub a: Vec<usize>,
    pub b: Vec<usize>,
    pub c: Vec<usize>,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct R1CS<F: PrimeField> {
    /// This is the C matrix
    pub c: Vec<Vec<F>>,
    /// This is the A matrix
    pub a: Vec<Vec<F>>,
    /// This is the B matrix
    pub b: Vec<Vec<F>>,
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

impl Constraint {
    pub fn new(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> Self {
        Constraint { a, b, c }
    }
}

impl ConstraintRaw {
    pub fn new(a: Vec<usize>, b: Vec<usize>, c: Vec<usize>) -> Self {
        ConstraintRaw { a, b, c }
    }

    pub fn to_constraint(&self, constraint_map: HashMap<usize, usize>) -> Constraint {
        let a = self.a.iter().map(|x| constraint_map[x]).collect();
        let b = self.b.iter().map(|x| constraint_map[x]).collect();
        let c = self.c.iter().map(|x| constraint_map[x]).collect();

        Constraint::new(a, b, c)
    }
}

impl ConstraintsWithLabelSize {
    pub fn new(constraints: Vec<Constraint>, label_size: usize) -> Self {
        ConstraintsWithLabelSize {
            constraints,
            label_size,
        }
    }

    pub fn to_r1cs_vec<F: PrimeField>(&self) -> R1CS<F> {
        let mut a = vec![vec![F::zero(); self.label_size + 1]; self.constraints.len()];
        let mut b = vec![vec![F::zero(); self.label_size + 1]; self.constraints.len()];
        let mut c = vec![vec![F::zero(); self.label_size + 1]; self.constraints.len()];

        for constraint in self.constraints.iter() {
            for i in 0..self.label_size {
                a[constraint.a[i]][i] = F::one();
                b[constraint.b[i]][i] = F::one();
                c[constraint.c[i]][i] = F::one();
            }
        }

        R1CS::new(a, b, c)
    }
}

impl<F: PrimeField> R1CS<F> {
    pub fn new(a: Vec<Vec<F>>, b: Vec<Vec<F>>, c: Vec<Vec<F>>) -> Self {
        Self { a, b, c }
    }

    pub fn check(&self, witness: Vec<F>) -> bool {
        check_init(
            self.a.clone(),
            self.b.clone(),
            self.c.clone(),
            witness.clone(),
        )
    }
}
