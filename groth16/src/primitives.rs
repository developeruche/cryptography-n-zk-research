use ark_ff::Field;
use polynomial::univariant::UnivariantPolynomial;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Witness<F: Field> {
    /// The public input to the circuit
    pub public_input: Vec<F>,
    /// The auxiliary input to the circuit
    pub auxiliary_input: Vec<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct R1CS<F: Field> {
    /// This is the C matrix
    c: Vec<Vec<F>>,
    /// This is the A matrix
    a: Vec<Vec<F>>,
    /// This is the B matrix
    b: Vec<Vec<F>>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAPPolysCoefficients<F: Field> {
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub c: Vec<F>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct QAP<F: Field> {
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
pub struct ToxicWaste<F: Field> {
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
pub struct TrustedSetup<F: Field> {
    toxic_waste: ToxicWaste<F>,
    number_of_constraints: usize,
}


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TrustedSetupExcecution<F: Field> {
    powers_of_tau_g1: Vec<F>,
    powers_of_tau_g2: Vec<F>,
    alpha_g1: F,
    beta_g1: F,
    delta_g1: F,
    beta_g2: F,
    delta_g2: F,
}




impl<F: Field> Witness<F> {
    pub fn render(&self) -> Vec<F> {
        let mut ren = self.public_input.clone();
        ren.extend(self.auxiliary_input.clone());
        ren
    }
}

impl<F: Field> TrustedSetup<F> {
    fn new(&self, toxic_waste: ToxicWaste<F>, number_of_constraints: usize) -> Self {
        Self {
            toxic_waste,
            number_of_constraints,
        }
    }
}