#![allow(non_snake_case)]

use ark_ff::PrimeField;
use plonk_core::primitives::PlonkishIntermediateRepresentation;
use polynomial::evaluation::{univariate::UnivariateEval, Domain};
use std::collections::HashMap;

use crate::{
    assembly::{eq_to_assembly, AssembleEquation, GateWire},
    utils::{get_product_key, roots_of_unity, Cell, Column},
};

pub struct Program<F: PrimeField> {
    pub constraints: Vec<AssembleEquation<F>>,
    pub group_order: u64,
}

impl<F: PrimeField> Program<F> {
    pub fn new(constraints: Vec<AssembleEquation<F>>, group_order: u64) -> Self {
        Program {
            constraints,
            group_order,
        }
    }

    pub fn new_with_string(constraints: Vec<String>, group_order: u64) -> Self {
        let mut parsed_constraints = vec![];
        for constraint in constraints {
            let parsed_constraint = eq_to_assembly(constraint);
            parsed_constraints.push(parsed_constraint);
        }
        Program {
            constraints: parsed_constraints,
            group_order,
        }
    }

    pub fn common_preproccessed_input(&self) -> PlonkishIntermediateRepresentation<F> {
        let (L, R, M, O, C) = self.make_gate_polynomials();
        let (S1, S2, S3) = self.make_s_polynomials();

        PlonkishIntermediateRepresentation {
            QM: M,
            QL: L,
            QR: R,
            QO: O,
            QC: C,
            S1,
            S2,
            S3,
            group_order: self.group_order,
        }
    }

    pub fn make_gate_polynomials(
        &self,
    ) -> (
        UnivariateEval<F>,
        UnivariateEval<F>,
        UnivariateEval<F>,
        UnivariateEval<F>,
        UnivariateEval<F>,
    ) {
        let mut L = vec![F::ZERO; self.group_order as usize];
        let mut R = vec![F::ZERO; self.group_order as usize];
        let mut M = vec![F::ZERO; self.group_order as usize];
        let mut O = vec![F::ZERO; self.group_order as usize];
        let mut C = vec![F::ZERO; self.group_order as usize];

        for (i, constraint) in self.constraints.iter().enumerate() {
            let gate = constraint.gate();
            L[i] = gate.L;
            R[i] = gate.R;
            M[i] = gate.M;
            O[i] = gate.O;
            C[i] = gate.C;
        }

        let domain = Domain::<F>::new(self.group_order as usize);
        (
            UnivariateEval::new(L, domain.clone()),
            UnivariateEval::new(R, domain.clone()),
            UnivariateEval::new(M, domain.clone()),
            UnivariateEval::new(O, domain.clone()),
            UnivariateEval::new(C, domain.clone()),
        )
    }

    pub fn make_s_polynomials(&self) -> (UnivariateEval<F>, UnivariateEval<F>, UnivariateEval<F>) {
        let mut variable_uses = HashMap::new();
        //The purpose of the loop is to put all the cells where the variable is located in variable_uses
        for (row, constraint) in self.constraints.iter().enumerate() {
            // Each layer loops, processes constraints, and uses the wires field.
            // L is placed in the first column, R in the second column, and O in the third column.
            for (column, variable) in constraint.wires.as_list().into_iter().enumerate() {
                variable_uses.entry(variable).or_insert(vec![]).push(Cell {
                    column: (column + 1).into(),
                    row,
                });
            }
        }
        // Now we have a mapping from the variable to the vec of all the cells it is in.
        // The following loop considers all empty cells.
        for row in self.constraints.len()..self.group_order as usize {
            for i in 1..=3 {
                variable_uses.entry(None).or_insert(vec![]).push(Cell {
                    column: i.into(),
                    row,
                })
            }
        }
        let mut s: HashMap<Column, Vec<F>> = HashMap::new();
        s.insert(
            Column::LEFT,
            roots_of_unity(self.group_order)
                .into_iter()
                .map(|element| element)
                .collect(),
        );
        s.insert(
            Column::RIGHT,
            roots_of_unity(self.group_order)
                .into_iter()
                .map(|element: F| element * F::from(2u32))
                .collect(),
        );
        s.insert(Column::OUTPUT, vec![F::ZERO; self.group_order as usize]);

        // exmaple
        // variable_uses = {"a":[Cell(1,3),Cell(3,4)],"b":[Cell(2,1)]
        for (_, uses) in variable_uses.iter() {
            // _ = "a"
            // uses = [Cell(1,3),Cell(3,4)]
            for (i, cell) in uses.iter().enumerate() {
                let next_i = (i + 1) % uses.len();
                let next_column = uses[next_i].column;
                let next_row = uses[next_i].row;
                if let Some(vec) = s.get_mut(&next_column) {
                    vec[next_row] = cell.label(self.group_order);
                }
            }
        }

        // Generate s1, s2, s3
        let mut s1 = None;
        let mut s2 = None;
        let mut s3 = None;
        for (key, vec) in s.into_iter() {
            let domain = Domain::<F>::new(self.group_order as usize);
            match key {
                Column::LEFT => s1 = Some(UnivariateEval::new(vec, domain)),
                Column::RIGHT => s2 = Some(UnivariateEval::new(vec, domain)),
                Column::OUTPUT => s3 = Some(UnivariateEval::new(vec, domain)),
            }
        }
        (s1.unwrap(), s2.unwrap(), s3.unwrap())
    }

    pub fn from_str(constraints: &str, group_order: u64) -> Self {
        let constraints = constraints
            .lines()
            .map(|line| eq_to_assembly(line.to_string()))
            .collect();
        Program::new(constraints, group_order)
    }

    pub fn coeffs(&self) -> Vec<HashMap<Option<String>, F>> {
        let mut coeffs = Vec::new();
        for constraint in self.constraints.iter() {
            coeffs.push(constraint.coeffs.clone());
        }
        coeffs
    }

    pub fn wires(&self) -> Vec<GateWire> {
        let mut wires = Vec::new();
        for constraint in self.constraints.iter() {
            wires.push(constraint.wires.clone());
        }
        return wires;
    }

    pub fn get_public_assignment(&self) -> Vec<Option<String>> {
        let mut no_more_allowed = false;
        let mut o = Vec::new();

        for coeff in self.coeffs() {
            if coeff.get(&Some("$public".to_string())).is_some() {
                if no_more_allowed {
                    panic!("Public var declarations must be at the top")
                }
                let mut var_name = Vec::new();
                for (key, _) in coeff.iter() {
                    if key.clone().unwrap().chars().next().unwrap() != '$' {
                        var_name.push(key.clone().unwrap());
                    }
                }
                o.push(Some(var_name.join("")));
            } else {
                no_more_allowed = true;
            }
        }

        o
    }

    /// Attempts to "run" the program to fill in any intermediate variable
    /// assignments, starting from the given assignments. Eg. if
    /// `starting_assignments` contains {'a': 3, 'b': 5}, and the first line
    /// says `c <== a * b`, then it fills in `c: 15`.
    pub fn fill_variable_assignments(
        &self,
        starting_assignments: HashMap<Option<String>, F>,
    ) -> HashMap<Option<String>, F> {
        let mut out: HashMap<Option<String>, F> = starting_assignments.clone();
        out.insert(None, F::ZERO);

        for constraint in self.constraints.iter() {
            let wires = constraint.wires.clone();
            let coeffs = constraint.coeffs.clone();

            let in_L = wires.L;
            let in_R = wires.R;
            let output = wires.O;
            let default_f_one = F::from(1u32);
            let out_coeff = coeffs
                .get(&Some("$output_coeff".to_string()))
                .unwrap_or(&default_f_one);
            let product_key = get_product_key(in_L.clone(), in_R.clone());

            if output.is_some() && (*out_coeff == F::ONE.neg() || *out_coeff == F::ONE) {
                let new_value = F::from(
                    *coeffs.get(&Some("".to_string())).unwrap_or(&F::ZERO)
                        + *out.get(&in_L).unwrap() * *coeffs.get(&in_L).unwrap_or(&F::ZERO)
                        + *out.get(&in_R).unwrap()
                            * *coeffs.get(&in_R).unwrap_or(&F::ZERO)
                            * if in_R != in_L { F::ONE } else { F::ZERO }
                        + *out.get(&in_L).unwrap()
                            * *out.get(&in_R).unwrap()
                            * coeffs.get(&product_key).unwrap_or(&F::ZERO),
                ) * out_coeff;

                if out.get(&output).is_some() {
                    if out.get(&output).unwrap() != &new_value {
                        panic!("Inconsistent assignment for variable {:?}", output);
                    }
                } else {
                    out.insert(output.clone(), new_value);
                }
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use circuits::{adapters::plonkish::plonkish_transpile, primitives::{Circuit, CircuitLayer, Gate, GateType}};

    #[test]
    fn test_make_s_polynomials() {
        //passed
        //L R  O
        //w 2w 3w
        //w^2 2w^2 3w^2

        //a b c
        //a e b
        let original_constriants = ["c <== a * b", "b <== a * e"];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        let program = Program::new(assembly_eqns, 8);
        let (s1, s2, s3) = program.make_s_polynomials();

        let unmoved_s1: Vec<_> = roots_of_unity(8);
        let unmoved_s2: Vec<_> = roots_of_unity(8)
            .into_iter()
            .map(|ele: Fr| ele * Fr::from(2))
            .collect();
        let unmoved_s3: Vec<_> = roots_of_unity(8)
            .into_iter()
            .map(|ele: Fr| ele * Fr::from(3))
            .collect();
        assert_eq!(s1.values[0], unmoved_s1[1]);

        assert_eq!(s2.values[0], unmoved_s3[1]);

        println!("s1:{:?}", s1);
        println!("s2:{:?}", s2);
        println!("s3:{:?}", s3);
    }

    #[test]
    fn test_make_gate_polynomials() {
        let original_constriants = ["e public", "c <== a * b", "e <== c * d"];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        let program = Program::new(assembly_eqns, 8);
        let (l, r, m, o, c) = program.make_gate_polynomials();
        println!("l:{:?}", l);
        println!("r:{:?}", r);
        println!("m:{:?}", m);
        println!("o:{:?}", o);
        println!("c:{:?}", c);
    }
    
    #[test]
    fn test_transpiler_compatibilty() {
        let original_constriants = ["v00 <== v10 * v11", "v10 <== v20 + v21", "v11 <== v22 * v23"];
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        
        
        let program = Program::new(assembly_eqns, 8);
        let (l, r, m, o, c) = program.make_gate_polynomials();
        let (s1, s2, s3) = program.make_s_polynomials();
        
        
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        
        let original_constriants = plonkish_transpile(&circuit);
        let mut assembly_eqns = Vec::new();
        for eq in original_constriants.iter() {
            let assembly_eqn = eq_to_assembly::<Fr>(eq.to_string());
            assembly_eqns.push(assembly_eqn);
        }
        
        let program = Program::new(assembly_eqns, 8);
        let (l_, r_, m_, o_, c_) = program.make_gate_polynomials();
        let (s1_, s2_, s3_) = program.make_s_polynomials();
        
        assert_eq!(l, l_);
        assert_eq!(r, r_);
        assert_eq!(m, m_);
        assert_eq!(o, o_);
        assert_eq!(c, c_);
        assert_eq!(s1, s1_);
        assert_eq!(s2, s2_);
        assert_eq!(s3, s3_);
    }
}
