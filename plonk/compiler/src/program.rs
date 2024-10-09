use std::collections::HashMap;

use ark_ff::PrimeField;
use plonk_core::primitives::CommonPreprocessedInput;
use polynomial::univariant::UnivariantPolynomial;

use crate::{
    assembly::{eq_to_assembly, AssembleEquation, GateWire},
    utils::{roots_of_unity, Cell, Column},
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
            let parsed_constraint = eq_to_assembly(&constraint);
            parsed_constraints.push(parsed_constraint);
        }
        Program {
            constraints: parsed_constraints,
            group_order,
        }
    }

    pub fn common_preproccessed_input(&self) -> CommonPreprocessedInput<F> {
        todo!()
    }

    pub fn make_gate_polynomials(
        &self,
    ) -> (
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
    ) {
        todo!()
    }

    pub fn make_s_polynomials(
        &self,
    ) -> (
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
        UnivariantPolynomial<F>,
    ) {
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
        // for (key, vec) in s.into_iter() {
        //     match key {
        //         Column::LEFT => s1 = Some(Polynomial::new(vec, Basis::Lagrange)),
        //         Column::RIGHT => s2 = Some(Polynomial::new(vec, Basis::Lagrange)),
        //         Column::OUTPUT => s3 = Some(Polynomial::new(vec, Basis::Lagrange)),
        //     }
        // }
        (s1.unwrap(), s2.unwrap(), s3.unwrap())
    }

    pub fn from_str(constraints: &str, group_order: u64) -> Self {
        let constraints = constraints
            .lines()
            .map(|line| eq_to_assembly(line))
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
        todo!()
    }
}
