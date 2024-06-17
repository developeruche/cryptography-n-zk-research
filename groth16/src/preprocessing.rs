use crate::{
    interfaces::R1CSProcessingInterface,
    primitives::{QAPPolysCoefficients, Witness, R1CS},
};
use ark_ff::PrimeField;

pub struct PreProcessor<F: PrimeField> {
    pub r1cs: R1CS<F>,
    pub withness: Witness<F>,
}

impl<F: PrimeField> PreProcessor<F> {
    pub fn new(r1cs: R1CS<F>, withness: Witness<F>) -> Self {
        Self { r1cs, withness }
    }
}

impl<F: PrimeField> R1CSProcessingInterface<F> for PreProcessor<F> {
    fn to_qap_poly_coefficients(&self) -> QAPPolysCoefficients<F> {
        // assert that all the r1cs components are of the same length
        // this checks if the number of constraints are equal
        assert!(
            self.r1cs.a.len() == self.r1cs.b.len() && self.r1cs.a.len() == self.r1cs.c.len(),
            "The R1CS components are not of the same length"
        );

        let mut new_a = vec![];
        let mut new_b = vec![];
        let mut new_c = vec![];

        let rows = self.r1cs.a.len();
        let columns = self.r1cs.a[0].len();

        for i in 0..columns {
            let mut a = vec![];
            let mut b = vec![];
            let mut c = vec![];

            for j in 0..rows {
                a.push(self.r1cs.a[j][i]);
                b.push(self.r1cs.b[j][i]);
                c.push(self.r1cs.c[j][i]);
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
