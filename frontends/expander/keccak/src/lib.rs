use crate::helpers::{copy_out_unaligned, keccak_f, xor_in};
use expander_compiler::frontend::{Config, Define, GF2Config, RootAPI, Variable};

mod constants;
mod helpers;
mod not_important;

#[derive(Clone)]
pub struct Keccak256Circuit<T> {
    pub p: [T; 64 * 8],
    out: [T; 256],
}

impl<B: Clone> ::expander_compiler::frontend::internal::DumpLoadTwoVariables<B>
    for Keccak256Circuit<B>
where
    B: ::expander_compiler::frontend::internal::DumpLoadVariables<B>,
{
    #[allow(unused_variables)]
    fn dump_into(&self, vars: &mut Vec<B>, public_vars: &mut Vec<B>) {
        for _x in self.p.iter() {
            _x.dump_into(vars);
        }
        for _x in self.out.iter() {
            _x.dump_into(public_vars);
        }
    }
    #[allow(unused_variables)]
    fn load_from(&mut self, vars: &mut &[B], public_vars: &mut &[B]) {
        for _x in self.p.iter_mut() {
            _x.load_from(vars);
        }
        for _x in self.out.iter_mut() {
            _x.load_from(public_vars);
        }
    }
    #[allow(unused_mut)]
    fn num_vars(&self) -> (usize, usize) {
        let mut cnt_sec = 0;
        let mut cnt_pub = 0;
        cnt_sec += 1 * (64 * 8);
        cnt_pub += 1 * 256;
        (cnt_sec, cnt_pub)
    }
}

impl<T: Default + Copy> Default for Keccak256Circuit<T> {
    fn default() -> Self {
        Self {
            p: [Default::default(); 64 * 8],
            out: [Default::default(); 256],
        }
    }
}

fn compute_keccak<C: Config, B: RootAPI<C>>(api: &mut B, p: &Vec<Variable>) -> Vec<Variable> {
    let mut ss = vec![vec![api.constant(0); 64]; 25];
    let mut new_p = p.clone();
    let mut append_data = vec![0; 136 - 64];
    append_data[0] = 1;
    append_data[135 - 64] = 0x80;
    for i in 0..136 - 64 {
        for j in 0..8 {
            new_p.push(api.constant(((append_data[i] >> j) & 1) as u32));
        }
    }
    let mut p = vec![vec![api.constant(0); 64]; 17];
    for i in 0..17 {
        for j in 0..64 {
            p[i][j] = new_p[i * 64 + j].clone();
        }
    }
    ss = xor_in(api, ss, p);
    ss = keccak_f(api, ss);
    copy_out_unaligned(ss, 136, 32)
}

impl Define<GF2Config> for Keccak256Circuit<Variable> {
    fn define<Builder: RootAPI<GF2Config>>(&self, api: &mut Builder) {
        // You can use api.memorized_simple_call for sub-circuits
        // Or use the function directly
        let out = api.memorized_simple_call(compute_keccak, &self.p.to_vec());
        //let out = compute_keccak(api, &self.p[i].to_vec());
        for i in 0..256 {
            api.assert_is_equal(out[i].clone(), self.out[i].clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use expander_compiler::{
        compile::CompileOptions,
        field::GF2,
        frontend::{CompileResult, compile},
    };
    use rand::{Rng, SeedableRng};
    use tiny_keccak::Hasher;

    #[test]
    #[ignore]
    fn keccak_gf2_debug() {
        let compile_result =
            compile(&Keccak256Circuit::default(), CompileOptions::default()).unwrap();
        let CompileResult {
            witness_solver,
            layered_circuit,
        } = compile_result;

        let mut assignment = Keccak256Circuit::<GF2>::default();
        let mut rng = rand::rngs::StdRng::seed_from_u64(1235);

        let mut data = vec![0u8; 64];
        for i in 0..64 {
            data[i] = rng.r#gen();
        }
        let mut hash = tiny_keccak::Keccak::v256();
        hash.update(&data);
        let mut output = [0u8; 32];
        hash.finalize(&mut output);
        for i in 0..64 {
            for j in 0..8 {
                assignment.p[i * 8 + j] = ((data[i] >> j) as u32 & 1).into();
            }
        }
        for i in 0..32 {
            for j in 0..8 {
                assignment.out[i * 8 + j] = ((output[i] >> j) as u32 & 1).into();
            }
        }

        let witness = witness_solver.solve_witness(&assignment).unwrap();
        println!("Witness: {:?}", witness);
        let res = layered_circuit.run(&witness);
        assert_eq!(res, vec![true]);
        println!("test 1 passed");
    }
}
