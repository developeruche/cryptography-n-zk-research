use std::default;

use expander_compiler::{
    declare_circuit,
    frontend::{internal::DumpLoadTwoVariables, BasicAPI, Define, M31Config, Variable},
};

pub struct Circuit<T> {
    pub x: T,
    y: T,
}

impl<B: Clone> ::expander_compiler::frontend::internal::DumpLoadTwoVariables<B> for Circuit<B>
where
    B: ::expander_compiler::frontend::internal::DumpLoadVariables<B>,
{
    #[allow(unused_variables)]
    fn dump_into(&self, vars: &mut Vec<B>, public_vars: &mut Vec<B>) {
        self.x.dump_into(vars);
        self.y.dump_into(vars);
    }
    #[allow(unused_variables)]
    fn load_from(&mut self, vars: &mut &[B], public_vars: &mut &[B]) {
        self.x.load_from(vars);
        self.y.load_from(vars);
    }
    #[allow(unused_mut)]
    fn num_vars(&self) -> (usize, usize) {
        let mut cnt_sec = 0;
        let mut cnt_pub = 0;
        cnt_sec += 1;
        cnt_sec += 1;
        (cnt_sec, cnt_pub)
    }
}
impl<T: Clone> Clone for Circuit<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl<T: Default + Copy> Default for Circuit<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Define<M31Config> for Circuit<Variable> {
    fn define<Builder: BasicAPI<M31Config>>(&self, api: &mut Builder) {
        api.assert_is_equal(self.x, self.y);
    }
}


#[cfg(test)]
mod test {
    use expander_compiler::{compile::CompileOptions, field::M31, frontend::{compile, CompileResult}};

    use super::*;
    
    #[test]
    fn example_full() {
        let CompileResult {
            witness_solver,
            layered_circuit,
        } = compile(&Circuit::default(), CompileOptions::default()).unwrap();
        assert_eq!(layered_circuit.layer_ids.len(), 2);
        let assignment = Circuit::<M31> {
            x: M31::from(123),
            y: M31::from(123),
        };
        let witness = witness_solver
            .solve_witness(&assignment)
            .unwrap();
        
        println!("Witness: {:?}", witness);
        
        let output = layered_circuit.run(&witness);
        assert_eq!(output, vec![true]);
    }
    
}