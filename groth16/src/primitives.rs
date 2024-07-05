use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use polynomial::{
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    univariant::UnivariantPolynomial,
    utils::compute_domain,
};
use rand::rngs::OsRng;

use crate::utils::check_init;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Witness<F: PrimeField> {
    /// The public input to the circuit
    pub public_input: Vec<F>,
    /// The auxiliary input to the circuit (private input)
    pub auxiliary_input: Vec<F>,
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAPPolysCoefficients<F: PrimeField> {
    pub a: Vec<Vec<F>>,
    pub b: Vec<Vec<F>>,
    pub c: Vec<Vec<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAPPolys<F: PrimeField> {
    pub a: Vec<UnivariantPolynomial<F>>,
    pub b: Vec<UnivariantPolynomial<F>>,
    pub c: Vec<UnivariantPolynomial<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAP<F: PrimeField> {
    /// This is the C matrix * witness in polynomial form
    pub cx: UnivariantPolynomial<F>,
    /// This is the A matrix * witness in polynomial form
    pub ax: UnivariantPolynomial<F>,
    /// This is the B matrix * witness in polynomial form
    pub bx: UnivariantPolynomial<F>,
    /// this is the t polynomial
    pub t: UnivariantPolynomial<F>,
    /// this is the h polynomial
    pub h: UnivariantPolynomial<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ToxicWaste<F: PrimeField> {
    pub alpha: F,
    pub beta: F,
    pub gamma: F,
    pub delta: F,
    pub tau: F,
}

/// This is the trusted setup
/// handles;
/// Circuit specific trusted setup and noc-specific trusted setup
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetup<P: Pairing> {
    phantom: std::marker::PhantomData<P>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProvingKey<P: Pairing> {
    pub alpha_g1: P::G1,
    pub beta_g1: P::G1,
    pub delta_g1: P::G2,
    pub powers_of_tau_g1: Vec<P::G1>, // from 0 to m - 1

    pub beta_g2: P::G2,
    pub delta_g2: P::G2,
    pub powers_of_tau_g2: Vec<P::G2>, // from 0 to m - 1
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct VerificationKey<P: Pairing> {
    pub alpha_g1: P::G1,

    pub beta_g2: P::G2,
    pub gamma_g2: P::G2,
    pub delta_g2: P::G2,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetupExcecution<P: Pairing> {
    pub powers_of_tau_g1: Vec<P::G1>, // from 0 to 2*m - 2
    pub powers_of_tau_g2: Vec<P::G2>, // from 0 to m - 1
    pub beta_g2: P::G2,
    pub alpha_g1: P::G1,
    pub beta_g1: P::G1,
    pub c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public: Vec<P::G1>,
    pub c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private: Vec<P::G1>,
    pub powers_of_tau_t_poly_delta_inverse_g1: Vec<P::G1>,
    pub gamma_g2: P::G2,
    pub delta_g2: P::G2,
    pub delta_g1: P::G1,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Proof<P: Pairing> {
    pub a: P::G1,
    pub b: P::G2,
    pub c: P::G1,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProofRands<F: PrimeField> {
    pub r: F,
    pub s: F,
}

impl<F: PrimeField> Witness<F> {
    pub fn new(public_input: Vec<F>, auxiliary_input: Vec<F>) -> Self {
        Self {
            public_input,
            auxiliary_input,
        }
    }

    pub fn render(&self) -> Vec<F> {
        let mut ren = self.public_input.clone();
        ren.extend(self.auxiliary_input.clone());
        ren
    }
}

impl<F: PrimeField> ToxicWaste<F> {
    pub fn random() -> Self {
        let rand_thread = &mut OsRng;

        let alpha = F::rand(rand_thread);
        let beta = F::rand(rand_thread);
        let gamma = F::rand(rand_thread);
        let delta = F::rand(rand_thread);
        let tau = F::rand(rand_thread);

        Self {
            alpha,
            beta,
            gamma,
            delta,
            tau,
        }
    }

    pub fn new(alpha: F, beta: F, gamma: F, delta: F, tau: F) -> Self {
        Self {
            alpha,
            beta,
            gamma,
            delta,
            tau,
        }
    }
}

impl<F: PrimeField> ProofRands<F> {
    pub fn random() -> Self {
        let rand_thread = &mut OsRng;

        let r = F::rand(rand_thread);
        let s = F::rand(rand_thread);

        Self { r, s }
    }

    pub fn new(r: F, s: F) -> Self {
        Self { r, s }
    }
}

impl<F: PrimeField> QAP<F> {
    pub fn new(
        cx: UnivariantPolynomial<F>,
        ax: UnivariantPolynomial<F>,
        bx: UnivariantPolynomial<F>,
        t: UnivariantPolynomial<F>,
        h: UnivariantPolynomial<F>,
    ) -> Self {
        Self { cx, ax, bx, t, h }
    }

    pub fn compute_ht(&self) -> UnivariantPolynomial<F> {
        self.h.clone() * self.t.clone()
    }

    pub fn qap_check(&self) -> bool {
        let ht = self.compute_ht();
        let lhs = self.ax.clone() * self.bx.clone();
        let check = lhs == ht + self.cx.clone();
        check
    }
}

impl<F: PrimeField> QAPPolysCoefficients<F> {
    pub fn new(a: Vec<Vec<F>>, b: Vec<Vec<F>>, c: Vec<Vec<F>>) -> Self {
        Self { a, b, c }
    }

    pub fn into_poly_rep(&self) -> QAPPolys<F> {
        let domain_lenght = self.a[0].len();
        let domain = compute_domain(domain_lenght);

        let a = self
            .a
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();
        let b = self
            .b
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();
        let c = self
            .c
            .iter()
            .map(|y| UnivariantPolynomial::interpolate(y.clone(), domain.clone()))
            .collect();

        QAPPolys { a, b, c }
    }
}

impl<P: Pairing> TrustedSetupExcecution<P> {
    pub fn new(
        powers_of_tau_g1: Vec<P::G1>,
        powers_of_tau_g2: Vec<P::G2>,
        beta_g2: P::G2,
        alpha_g1: P::G1,
        beta_g1: P::G1,
        c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public: Vec<P::G1>,
        c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private: Vec<P::G1>,
        powers_of_tau_t_poly_delta_inverse_g1: Vec<P::G1>,
        gamma_g2: P::G2,
        delta_g2: P::G2,
        delta_g1: P::G1,
    ) -> Self {
        Self {
            powers_of_tau_g1,
            powers_of_tau_g2,
            beta_g2,
            alpha_g1,
            beta_g1,
            c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_public,
            c_tau_plus_beta_a_tau_plus_alpha_b_tau_g1_private,
            powers_of_tau_t_poly_delta_inverse_g1,
            gamma_g2,
            delta_g2,
            delta_g1,
        }
    }

    pub fn get_n_powers_of_tau_g1(&self, n: usize) -> Vec<P::G1> {
        self.powers_of_tau_g1[..n].to_vec()
    }
}

impl<F: PrimeField> QAPPolys<F> {
    pub fn new(
        a: Vec<UnivariantPolynomial<F>>,
        b: Vec<UnivariantPolynomial<F>>,
        c: Vec<UnivariantPolynomial<F>>,
    ) -> Self {
        Self { a, b, c }
    }
}

impl<F: PrimeField> R1CS<F> {
    pub fn new(a: Vec<Vec<F>>, b: Vec<Vec<F>>, c: Vec<Vec<F>>) -> Self {
        Self { a, b, c }
    }

    pub fn check(&self, witness: Vec<F>) -> bool {
        check_init(self.a.clone(), self.b.clone(), self.c.clone(), witness.clone())
    }
}
