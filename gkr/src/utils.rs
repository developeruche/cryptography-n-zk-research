use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::multilinear::ComposedMultilinear,
    interface::{MultilinearPolynomialInterface, PolynomialInterface},
    multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::{
    composed::{multicomposed::MultiComposedProver, ComposedSumCheckProof},
    interface::MultiComposedProverInterface,
};

pub fn gen_w_mle<F: PrimeField>(evals: &[Vec<F>], layer_index: usize) -> Multilinear<F> {
    // see if the layer index is out of bounds
    if layer_index >= evals.len() {
        panic!("Layer index out of bounds");
    }

    Multilinear::interpolate(&evals[layer_index])
}

pub fn gen_l<F: PrimeField>(
    b: &[F],
    c: &[F],
) -> Result<Vec<UnivariantPolynomial<F>>, &'static str> {
    // perfroming some santiy checks
    if b.len() != c.len() {
        return Err("Length of b and c must be the same");
    }

    Ok(b.iter()
        .zip(c.iter())
        .map(|(b, c)| {
            let mut coeffs = vec![*b, *c - b];
            UnivariantPolynomial::new(coeffs)
        })
        .collect())
}

pub fn evaluate_l<F: PrimeField>(l: &[UnivariantPolynomial<F>], x: F) -> Vec<F> {
    l.iter().map(|l_i| l_i.evaluate(&x)).collect()
}

pub fn gen_q<F: PrimeField>(
    l: &[UnivariantPolynomial<F>],
    w: Multilinear<F>,
) -> Result<UnivariantPolynomial<F>, &'static str> {
    // performing some sanity checks
    if l.len() != w.num_vars() {
        return Err("Length of l and w must be the same");
    }

    todo!()
}

pub fn perform_gkr_sumcheck_layer_one<F: PrimeField>(
    layer_claim: &F,
    layer_one_r: Vec<F>,
    add_mle: &Multilinear<F>,
    mul_mle: &Multilinear<F>,
    w_mle: &Multilinear<F>,
    transcript: &mut FiatShamirTranscript,
    sum_check_proofs: &mut Vec<ComposedSumCheckProof<F>>,
    w_i_b: &mut Vec<F>,
    w_i_c: &mut Vec<F>,
) -> (F, Vec<F>, Vec<F>) {
    let number_of_round = layer_one_r.len();

    // add(r, b, c) ---> add(b, c)
    let add_b_c = add_mle.partial_evaluations(layer_one_r.clone(), vec![0; number_of_round]);
    // mul(r, b, c) ---> mul(b, c)
    let mul_b_c = mul_mle.partial_evaluations(layer_one_r, vec![0; number_of_round]);

    let wb = w_mle.clone();
    let wc = w_mle.clone();

    // w_i(b) + w_i(c)
    let wb_add_wc = wb.add_distinct(&wc);
    // w_i(b) * w_i(c)
    let wb_mul_wc = wb.mul_distinct(&wc);

    //  add(b, c)(w_i(b) + w_i(c))
    let f_b_c_add_section = ComposedMultilinear::new(vec![add_b_c, wb_add_wc]);
    // mul(b, c)(w_i(b) * w_i(c))
    let f_b_c_mul_section = ComposedMultilinear::new(vec![mul_b_c, wb_mul_wc]);

    // f(b, c) = add(r, b, c)(w_i(b) + w_i(c)) + mul(r, b, c)(w_i(b) * w_i(c))
    let f_b_c = vec![f_b_c_add_section, f_b_c_mul_section];

    // this prover that the `claim` is the result of the evalution of the preivous layer
    let (sumcheck_proof, random_challenges) =
        MultiComposedProver::sum_check_proof_without_initial_polynomial(&f_b_c);
    

    
    transcript.append(sumcheck_proof.to_bytes());
    sum_check_proofs.push(sumcheck_proof);

    let (rand_b, rand_c) = random_challenges.split_at(random_challenges.len() / 2);

    let eval_w_i_b = wb.evaluate(&rand_b.to_vec()).unwrap();
    let eval_w_i_c = wc.evaluate(&rand_c.to_vec()).unwrap();

    w_i_b.push(eval_w_i_b);
    w_i_c.push(eval_w_i_c);

    (layer_claim.clone(), rand_b.to_vec(), rand_c.to_vec())
}
