use crate::{
    interfaces::{PreProcessorInterface, QAPPolysCoefficientsInterface, R1CSProcessingInterface},
    primitives::{QAPPolysCoefficients, Witness, QAP, R1CS},
    utils::generate_t_poly,
};
use ark_ff::PrimeField;
use polynomial::univariant::UnivariantPolynomial;

pub struct PreProcessor<F: PrimeField> {
    pub r1cs: R1CS<F>,
    pub witness: Witness<F>,
}

impl<F: PrimeField> PreProcessor<F> {
    pub fn new(r1cs: R1CS<F>, witness: Witness<F>) -> Self {
        Self { r1cs, witness }
    }
}

impl<F: PrimeField> R1CSProcessingInterface<F> for R1CS<F> {
    fn to_qap_poly_coefficients(&self) -> QAPPolysCoefficients<F> {
        // assert that all the r1cs components are of the same length
        // this checks if the number of constraints are equal
        assert!(
            self.a.len() == self.b.len() && self.a.len() == self.c.len(),
            "The R1CS components are not of the same length"
        );

        let mut new_a = vec![];
        let mut new_b = vec![];
        let mut new_c = vec![];

        let rows = self.a.len();
        let columns = self.a[0].len();

        for i in 0..columns {
            let mut a = vec![];
            let mut b = vec![];
            let mut c = vec![];

            for j in 0..rows {
                a.push(self.a[j][i]);
                b.push(self.b[j][i]);
                c.push(self.c[j][i]);
            }

            new_a.push(a);
            new_b.push(b);
            new_c.push(c);
        }

        QAPPolysCoefficients {
            a: new_a,
            b: new_b,
            c: new_c,
        }
    }
}

impl<F: PrimeField> QAPPolysCoefficientsInterface<F> for QAPPolysCoefficients<F> {
    fn to_qap_polynomials(&self, witness: Vec<F>) -> QAP<F> {
        let polys = self.into_poly_rep();
        let cx = polys
            .c
            .iter()
            .zip(witness.iter())
            .map(|(p, w)| p.clone() * w.clone())
            .fold(UnivariantPolynomial::one(), |acc, x| acc + x);
        let ax = polys
            .a
            .iter()
            .zip(witness.iter())
            .map(|(p, w)| p.clone() * w.clone())
            .fold(UnivariantPolynomial::one(), |acc, x| acc + x);
        let bx = polys
            .b
            .iter()
            .zip(witness.iter())
            .map(|(p, w)| p.clone() * w.clone())
            .fold(UnivariantPolynomial::one(), |acc, x| acc + x);
        
        let t = generate_t_poly::<F>(witness.len());
        let h = ((ax.clone() * bx.clone()) - cx.clone()) / t.clone();

        QAP::new(cx, ax, bx, t, h)
    }
}

impl<F: PrimeField> PreProcessorInterface<F> for PreProcessor<F> {
    fn preprocess(&self) -> QAP<F> {
        let qap_poly_coefficients = self.r1cs.to_qap_poly_coefficients();
        qap_poly_coefficients.to_qap_polynomials(self.witness.render())
    }
}
