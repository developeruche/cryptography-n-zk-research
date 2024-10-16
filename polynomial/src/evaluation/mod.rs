//! This a representation of different polynomial structurces in their
//! Evaluation form.
//! This domain implemenation took a lot of inspiration from arkworks!
use ark_ff::{FftField, PrimeField};
pub mod univariate;
pub mod utils;

pub struct Domain<F: FftField> {
    /// This is a const size of the domain
    pub size: u64,
    /// This is the generator of the domain, ofter regarded as the root of unity (omega)
    pub generator: F,
    /// This is the inverse of the group generator
    pub group_gen_inverse: F,
}

impl<F: PrimeField> Domain<F> {
    /// This function is used to build a domain that would be large enough to
    /// perform evaluations of a polynomial of `num_of_coeffs` coefficients.
    pub fn new(num_of_coeffs: usize) -> Self {
        let size = if num_of_coeffs.is_power_of_two() {
            num_of_coeffs
        } else {
            num_of_coeffs.checked_next_power_of_two().unwrap()
        } as u64;

        let generator = F::get_root_of_unity(size).unwrap();
        let group_gen_inverse = generator.inverse().unwrap();

        Domain {
            size,
            generator,
            group_gen_inverse,
        }
    }

    /// This function is used to obtain the coset of the domain
    pub fn get_coset(&self, offest: usize) -> Self {
        todo!()
    }

    /// This function returns the roots of unity
    pub fn get_roots_of_unity(&self) -> Vec<F> {
        todo!()
    }

    /// This function is used to get the root of unity
    pub fn get_root_of_unity(&self) -> F {
        self.generator
    }

    pub fn fft_in_place(&self, coeffs: &mut Vec<F>) {
        if coeffs.len() == 1 {
            return;
        }

        coeffs.resize(self.size as usize, F::ZERO);
        self.fft_in_place_internal(coeffs, false);
    }
    
    pub fn ifft_in_place(&self, evals: &mut Vec<F>) {
        if evals.len() == 1 {
            return;
        }

        evals.resize(self.size as usize, F::ZERO);
        self.fft_in_place_internal(evals, true);
    }

    pub fn fft_in_place_internal(&self, coeffs: &mut Vec<F>, is_inverse: bool) -> Vec<F> {
        let len_of_coeffs = coeffs.len();
        if len_of_coeffs == 1 {
            return coeffs.clone();
        }

        let (mut even, mut odd) = utils::split_odd_even::<F>(&coeffs);

        let (y_even, y_odd) = (
            self.fft_in_place_internal(&mut even, is_inverse),
            self.fft_in_place_internal(&mut odd, is_inverse),
        );

        let mut y = vec![F::ZERO; len_of_coeffs];
        let w = if is_inverse {
            self.group_gen_inverse
        } else {
            self.generator
        };

        for j in 0..len_of_coeffs / 2 {
            y[j] = y_even[j] + (y_odd[j] * w.pow(&[j as u64]));
            y[j + len_of_coeffs / 2] = y_even[j] - (y_odd[j] * w.pow(&[j as u64]));
        }

        y
    }
}
