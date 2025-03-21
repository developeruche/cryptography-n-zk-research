//! Verifier module for the sumcheck protocol of the virtual polynomial.
use super::{SumCheckProverMessage, SumCheckSubClaim, SumCheckVerifier};
use ark_ff::PrimeField;
use fiat_shamir::{FiatShamirTranscript, TranscriptInterface};

#[derive(Clone, Default, Debug)]
pub struct VirtualVerifier<F: PrimeField> {
    pub(crate) round: usize,
    pub(crate) num_vars: usize,
    pub(crate) max_degree: usize,
    pub(crate) finished: bool,
    /// a list storing the univariate polynomial in evaluation form sent by the
    /// prover at each round
    pub(crate) polynomials_received: Vec<Vec<F>>,
    /// a list storing the randomness sampled by the verifier at each round
    pub(crate) challenges: Vec<F>,
}

impl<F: PrimeField> SumCheckVerifier<F> for VirtualVerifier<F> {
    type VPAuxInfo = (usize, usize);
    type ProverMessage = SumCheckProverMessage<F>;
    type Challenge = F;
    type Transcript = FiatShamirTranscript;
    type SumCheckSubClaim = SumCheckSubClaim<F>;

    fn verifier_init(index_info: &Self::VPAuxInfo) -> Self {
        Self {
            round: 0,
            num_vars: index_info.0,
            max_degree: index_info.1,
            finished: false,
            polynomials_received: Vec::with_capacity(index_info.0),
            challenges: Vec::with_capacity(index_info.0),
        }
    }

    fn verify_round_and_update_state(
        &mut self,
        prover_msg: &Self::ProverMessage,
        transcript: &mut Self::Transcript,
    ) -> Result<Self::Challenge, anyhow::Error> {
        if self.finished {
            return Err(anyhow::anyhow!("Verifier is finished"));
        }

        transcript.append(b"Internal round".to_vec());
        let challenge: F = transcript.sample_as_field_element();
        self.polynomials_received
            .push(prover_msg.evaluations.to_vec());

        if self.round == self.num_vars {
            self.finished = true;
        } else {
            self.round += 1;
        }

        Ok(challenge)
    }

    fn check_and_generate_subclaim(
        &self,
        asserted_sum: &F,
    ) -> Result<Self::SumCheckSubClaim, anyhow::Error> {
        if !self.finished {
            return Err(anyhow::anyhow!(
                "Incorrect verifier state: Verifier has not finished."
            ));
        }

        if self.polynomials_received.len() != self.num_vars {
            return Err(anyhow::anyhow!("insufficient rounds"));
        }

        let mut expected_vec = self
            .polynomials_received
            .clone()
            .into_iter()
            .zip(self.challenges.clone().into_iter())
            .map(|(evaluations, challenge)| {
                if evaluations.len() != self.max_degree + 1 {
                    return Err(anyhow::anyhow!(
                        "incorrect number of evaluations: {} vs {}",
                        evaluations.len(),
                        self.max_degree + 1
                    ));
                }
                interpolate_uni_poly::<F>(&evaluations, challenge)
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        // insert the asserted_sum to the first position of the expected vector
        expected_vec.insert(0, *asserted_sum);

        for (evaluations, &expected) in self
            .polynomials_received
            .iter()
            .zip(expected_vec.iter())
            .take(self.num_vars)
        {
            // the deferred check during the interactive phase:
            // 1. check if the received 'P(0) + P(1) = expected`.
            if evaluations[0] + evaluations[1] != expected {
                return Err(anyhow::anyhow!(
                    "Prover message is not consistent with the claim."
                ));
            }
        }

        Ok(SumCheckSubClaim {
            point: self.challenges.clone(),
            // the last expected value (not checked within this function) will be included in the
            // subclaim
            expected_evaluation: expected_vec[self.num_vars],
        })
    }
}

fn interpolate_uni_poly<F: PrimeField>(p_i: &[F], eval_at: F) -> Result<F, anyhow::Error> {
    let len = p_i.len();
    let mut evals = vec![];
    let mut prod = eval_at;
    evals.push(eval_at);

    // `prod = \prod_{j} (eval_at - j)`
    for e in 1..len {
        let tmp = eval_at - F::from(e as u64);
        evals.push(tmp);
        prod *= tmp;
    }
    let mut res = F::zero();
    if p_i.len() <= 20 {
        let last_denominator = F::from(u64_factorial(len - 1));
        let mut ratio_numerator = 1i64;
        let mut ratio_denominator = 1u64;

        for i in (0..len).rev() {
            let ratio_numerator_f = if ratio_numerator < 0 {
                -F::from((-ratio_numerator) as u64)
            } else {
                F::from(ratio_numerator as u64)
            };

            res += p_i[i] * prod * F::from(ratio_denominator)
                / (last_denominator * ratio_numerator_f * evals[i]);

            // compute denom for the next step is current_denom * (len-i)/i
            if i != 0 {
                ratio_numerator *= -(len as i64 - i as i64);
                ratio_denominator *= i as u64;
            }
        }
    } else if p_i.len() <= 33 {
        let last_denominator = F::from(u128_factorial(len - 1));
        let mut ratio_numerator = 1i128;
        let mut ratio_denominator = 1u128;

        for i in (0..len).rev() {
            let ratio_numerator_f = if ratio_numerator < 0 {
                -F::from((-ratio_numerator) as u128)
            } else {
                F::from(ratio_numerator as u128)
            };

            res += p_i[i] * prod * F::from(ratio_denominator)
                / (last_denominator * ratio_numerator_f * evals[i]);

            // compute denom for the next step is current_denom * (len-i)/i
            if i != 0 {
                ratio_numerator *= -(len as i128 - i as i128);
                ratio_denominator *= i as u128;
            }
        }
    } else {
        let mut denom_up = field_factorial::<F>(len - 1);
        let mut denom_down = F::one();

        for i in (0..len).rev() {
            res += p_i[i] * prod * denom_down / (denom_up * evals[i]);

            // compute denom for the next step is current_denom * (len-i)/i
            if i != 0 {
                denom_up *= -F::from((len - i) as u64);
                denom_down *= F::from(i as u64);
            }
        }
    }

    Ok(res)
}

fn u64_factorial(a: usize) -> u64 {
    let mut res = 1u64;
    for i in 2..=a {
        res *= i as u64;
    }
    res
}

fn u128_factorial(a: usize) -> u128 {
    let mut res = 1u128;
    for i in 2..=a {
        res *= i as u128;
    }
    res
}

fn field_factorial<F: PrimeField>(a: usize) -> F {
    let mut res = F::one();
    for i in 2..=a {
        res *= F::from(i as u64);
    }
    res
}
