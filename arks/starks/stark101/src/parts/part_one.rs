//! Part One of the Stark101 course
//! - Low degree Extension
//! - Commitment
use m_tree::{DefaultMerkleTree, DefaultHasher, Hasher};
use polynomial::{ark_ff::PrimeField, interface::{PolynomialInterface, UnivariantPolynomialInterface}, univariant::UnivariantPolynomial};
use ark_ff::BigInteger;



pub fn part_one<F: PrimeField>() {
    // Trace of the computation
    // a_0, a_1, a_2, a_3,..., a_n
    // 1, g, g^2, g^3,..., g^n
    // f(g) = a;
    let trace_target = 1024;
    let mut trace: Vec<F> = Vec::new();
    trace.push(F::one());
    trace.push(F::from(3141592u32));

    loop {
        if trace.len() == trace_target {
            break;
        }
        // next element is equal to the sum of the squares of the previous two elements
        let next = trace[trace.len() - 1].square() + trace[trace.len() - 2].square();
        trace.push(next);
    }
    
    // print first 5 and last 5 elements of the trace
    for i in 0..5 {
        println!("trace[{}] = {}", i, trace[i]);
    }
    for i in (trace.len() - 5)..trace.len() {
        println!("trace[{}] = {}", i, trace[i]);
    }

    let t = (3 * 2u64.pow(20)) as u64;
    let g = F::GENERATOR.pow([t]);
    let domain = (0..1024).map(|i| g.pow([i as u64])).collect::<Vec<F>>();
    let f_of_g = UnivariantPolynomial::interpolate(trace.clone(), domain.clone());
    let root_of_unity = F::GENERATOR;
    let coset_factor = root_of_unity.pow([t/8192]);
    let h_s = (0..8192).map(|i| coset_factor.pow([i as u64])).collect::<Vec<F>>();
    let ext_domain = h_s.iter().map(|h| *h * g).collect::<Vec<F>>();
    let low_degree_extension = ext_domain.iter().map(|x| f_of_g.evaluate(x)).collect::<Vec<F>>();
    let low_degree_extension_leaves = low_degree_extension
        .iter()
        .map(|x| {
            let bytes = x.into_bigint().to_bytes_le();
            DefaultHasher::hash(&bytes)
        })
        .collect::<Vec<[u8; 32]>>();
    let commitment = DefaultMerkleTree::from_leaves(&low_degree_extension_leaves);
    
    println!("Commitment: {:?}", commitment.root());
}











