use ark_ec::pairing::Pairing;
use ark_ff::PrimeField;
use polynomial::{
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    univariant::UnivariantPolynomial,
    utils::compute_domain,
};
use rand::rngs::OsRng;

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
    alpha: F,
    beta: F,
    gamma: F,
    delta: F,
    tau: F,
}

/// This is the trusted setup
/// handles;
/// Circuit specific trusted setup and noc-specific trusted setup
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetup<P: Pairing> {
    toxic_waste: ToxicWaste<P::ScalarField>,
    number_of_constraints: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProvingKey<F: PrimeField> {
    alpha_g1: F,
    beta_g1: F,
    delta_g1: F,
    powers_of_tau_g1: Vec<F>, // from 0 to m - 1

    beta_g2: F,
    delta_g2: F,
    powers_of_tau_g2: Vec<F>, // from 0 to m - 1
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct VerificationKey<F: PrimeField> {
    alpha_g1: F,

    beta_g2: F,
    gamma_g2: F,
    delta_g2: F,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetupExcecution<F: PrimeField> {
    powers_of_tau_g1: Vec<F>,       // from 0 to 2*m - 2
    powers_of_tau_g2: Vec<F>,       // from 0 to m - 1
    powers_of_tau_g1_alpha: Vec<F>, // from 0 to m - 1
    powers_of_tau_g1_beta: Vec<F>,  // from 0 to m - 1
    beta_g2: F,
    delta_g2: F,
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

impl<P: Pairing> TrustedSetup<P> {
    pub fn new(&self, toxic_waste: ToxicWaste<P::ScalarField>, number_of_constraints: usize) -> Self {
        Self {
            toxic_waste,
            number_of_constraints,
        }
    }

    pub fn new_with_random(&self, number_of_constraints: usize) -> Self {
        let toxic_waste = ToxicWaste::random();
        Self {
            toxic_waste,
            number_of_constraints,
        }
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
        let check_1 = lhs == ht + self.cx.clone();
        let check_2 = self.ax.evaluate(&F::from(1u32)) * self.bx.evaluate(&F::from(1u32))
            == self.cx.evaluate(&F::from(1u32));
        check_1 && check_2
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

impl<F: PrimeField> TrustedSetupExcecution<F> {
    pub fn get_n_powers_of_tau_g1(&self, n: usize) -> Vec<F> {
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
