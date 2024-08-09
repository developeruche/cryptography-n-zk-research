use ark_ff::PrimeField;
use fiat_shamir::{interface::TranscriptInterface, FiatShamirTranscript};
use polynomial::{
    composed::multilinear::ComposedMultilinear,
    interface::{MultilinearPolynomialInterface, PolynomialInterface},
    multilinear::Multilinear,
    univariant::UnivariantPolynomial,
};
use sum_check::{
    composed::{
        multicomposed::{MultiComposedProver, MultiComposedVerifier},
        ComposedSumCheckProof,
    },
    interface::{MultiComposedProverInterface, MultiComposedVerifierInterface},
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
            let coeffs = vec![*b, *c - b];
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
    claim: F,
    layer_one_r: Vec<F>,
    add_mle: &Multilinear<F>,
    mul_mle: &Multilinear<F>,
    w_mle: &Multilinear<F>,
    transcript: &mut FiatShamirTranscript,
    sum_check_proofs: &mut Vec<ComposedSumCheckProof<F>>,
    w_i_b: &mut Vec<F>,
    w_i_c: &mut Vec<F>,
) -> (F, Vec<F>, Vec<F>, F, F) {
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
        MultiComposedProver::sum_check_proof_without_initial_polynomial(&f_b_c, &claim);

    transcript.append(sumcheck_proof.to_bytes());
    sum_check_proofs.push(sumcheck_proof);

    let (rand_b, rand_c) = random_challenges.split_at(random_challenges.len() / 2);

    let eval_w_i_b = wb.evaluate(&rand_b.to_vec()).unwrap();
    let eval_w_i_c = wc.evaluate(&rand_c.to_vec()).unwrap();

    w_i_b.push(eval_w_i_b);
    w_i_c.push(eval_w_i_c);

    let alpha: F = transcript.sample_as_field_element();
    let beta: F = transcript.sample_as_field_element();

    let new_claim = alpha * eval_w_i_b + beta * eval_w_i_c;

    (new_claim, rand_b.to_vec(), rand_c.to_vec(), alpha, beta)
}

pub fn verifiy_gkr_sumcheck_layer_one<F: PrimeField>(
    layer_one_expected_claim: &F, // this should be the excecution of layer zero
    layer_one_sum_check_proof: &ComposedSumCheckProof<F>, // this is the sum check proof from layer one excecuted by the prover
    transcript: &mut FiatShamirTranscript,
    w_b: F,
    w_c: F,
    n_r: Vec<F>,
    add_mle: &Multilinear<F>,
    mul_mle: &Multilinear<F>,
) -> (bool, F) {
    // check if the claim is the same as the expected claim
    if *layer_one_expected_claim != layer_one_sum_check_proof.sum {
        println!("Invalid sumcheck proof");
        (false, F::ZERO);
    }

    transcript.append(layer_one_sum_check_proof.to_bytes());

    let intermidate_claim_check =
        MultiComposedVerifier::verify_except_last_check(&layer_one_sum_check_proof);

    // performing sum check last check
    let mut r_b_c = n_r;
    r_b_c.extend_from_slice(&intermidate_claim_check.random_challenges);

    let add_b_c = add_mle.evaluate(&r_b_c).unwrap();
    let mul_b_c = mul_mle.evaluate(&r_b_c).unwrap();

    let add_section = add_b_c * (w_b + w_c);
    let mul_section = mul_b_c * (w_b * w_c);

    let f_b_c_eval = add_section + mul_section;

    if f_b_c_eval != intermidate_claim_check.claimed_sum {
        println!("Invalid sumcheck proof");
        return (false, F::ZERO);
    }

    let alpha: F = transcript.sample_as_field_element();
    let beta: F = transcript.sample_as_field_element();

    let new_claim = alpha * w_b + beta * w_c;

    (true, new_claim)
}
