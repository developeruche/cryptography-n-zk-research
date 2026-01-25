//! This a representation of different polynomial structurces in their
//! Evaluation form.
//! This domain implemenation took a lot of inspiration from arkworks!
use ark_ff::{FftField, PrimeField};
use utils::serial_fft;
pub mod univariate;
pub mod utils;

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Domain<F: FftField> {
    /// This is a const size of the domain
    pub(crate) size: u64,
    /// This is the generator of the domain, ofter regarded as the root of unity (omega)
    pub(crate) generator: F,
    /// This is the offset of the domain (multiplicative coset factor)
    pub(crate) offset: F,
    /// This is the inverse of the group generator
    pub(crate) group_gen_inverse: F,
    /// This is the inverse of the group size
    pub(crate) group_size_inverse: F,
}

// implemenat the display trait for the domain
impl<F: PrimeField> std::fmt::Display for Domain<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Domain: size = {}, generator = {}, offset = {}, group_gen_inverse = {}",
            self.size, self.generator, self.offset, self.group_gen_inverse
        )
    }
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
        let group_size_inverse = F::from(size).inverse().unwrap();

        Domain {
            size,
            generator,
            offset: F::one(),
            group_gen_inverse,
            group_size_inverse,
        }
    }

    /// This function is used to obtain the coset of the domain
    pub fn get_coset(&self, offset: F) -> Self {
        Domain {
            size: self.size,
            generator: self.generator,
            offset: self.offset * offset,
            group_gen_inverse: self.group_gen_inverse,
            group_size_inverse: self.group_size_inverse,
        }
    }

    /// This function returns the roots of unity (elements of the domain)
    pub fn get_roots_of_unity(&self) -> Vec<F> {
        // Initialize a vector to store the roots of unity
        let mut roots = Vec::with_capacity(self.size as usize);

        // Start with the offset (which is 1 for the base domain)
        let mut current = self.offset;

        // Get the generator (root of unity) omega
        let omega = self.generator;

        // Iterate through to calculate the powers of omega
        for _ in 0..self.size {
            roots.push(current);
            current *= omega; // Move to the next power of omega
        }

        roots
    }

    /// This function gets inverse roots of unity
    pub fn get_inv_roots_of_unity(&self) -> Vec<F> {
        // Initialize a vector to store the roots of unity
        let mut roots = Vec::with_capacity(self.size as usize);

        // Start with the inverse of the offset
        let mut current = self.offset.inverse().unwrap();

        // Get the generator (root of unity) omega
        let omega = self.group_gen_inverse;

        // Iterate through to calculate the powers of omega
        for _ in 0..self.size {
            roots.push(current);
            current *= omega; // Move to the next power of omega
        }

        roots
    }

    /// This function is used to get the root of unity
    pub fn get_root_of_unity(&self) -> F {
        self.generator
    }

    pub fn fft(&self, coeffs: &Vec<F>) -> Vec<F> {
        let mut coeffs = coeffs.clone();
        self.fft_internal(&mut coeffs);
        coeffs
    }

    pub fn ifft(&self, evals: &Vec<F>) -> Vec<F> {
        let mut evals = evals.clone();
        self.ifft_internal(&mut evals);
        evals
    }

    pub fn fft_internal(&self, coeffs: &mut Vec<F>) {
        coeffs.resize(self.size as usize, F::zero());
        
        // If offset is not 1, we need to multiply by offset^i before FFT
        if self.offset != F::one() {
            let mut offset_pow = F::one();
            for coeff in coeffs.iter_mut() {
                *coeff *= offset_pow;
                offset_pow *= self.offset;
            }
        }
        
        serial_fft(coeffs, self.generator, self.size.trailing_zeros());
    }

    pub fn ifft_internal(&self, evals: &mut Vec<F>) {
        evals.resize(self.size as usize, F::zero());
        serial_fft(evals, self.group_gen_inverse, self.size.trailing_zeros());

        // scaling down the resulting coefficients
        let scaling_factor = self.group_size_inverse;
        
        // If offset is not 1, we need to divide by offset^i (multiply by offset^-i)
        // and also scale by group_size_inverse
        if self.offset != F::one() {
             let offset_inv = self.offset.inverse().unwrap();
             let mut offset_pow_inv = F::one();
             
             for eval in evals.iter_mut() {
                 *eval *= scaling_factor * offset_pow_inv;
                 offset_pow_inv *= offset_inv;
             }
        } else {
             evals
                .iter_mut()
                .for_each(|eval| *eval *= scaling_factor); //TODO: This can be parallelized!
        }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn generator(&self) -> F {
        self.generator
    }

    pub fn group_gen_inverse(&self) -> F {
        self.group_gen_inverse
    }

    pub fn offset(&self) -> F {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::One;
    use ark_test_curves::bls12_381::Fr;

    #[test]
    fn test_domain_new() {
        let domain = Domain::<Fr>::new(10);
        assert_eq!(domain.size, 16);
        assert_eq!(
            domain.generator.to_string(),
            String::from(
                "14788168760825820622209131888203028446852016562542525606630160374691593895118"
            )
        );
        assert_eq!(
            domain.group_gen_inverse.to_string(),
            String::from(
                "26753076894533791554649012143113393549300550745003194222677083919072199473480"
            )
        );
    }

    #[test]
    fn test_coset_properties() {
        let domain = Domain::<Fr>::new(8);
        assert_eq!(domain.offset, Fr::one());

        // Create a coset with offset g (generator)
        let offset = domain.generator;
        let coset = domain.get_coset(offset);

        assert_eq!(coset.offset, offset);
        assert_eq!(coset.size, domain.size);
        assert_eq!(coset.generator, domain.generator);

        let roots = domain.get_roots_of_unity();
        let coset_roots = coset.get_roots_of_unity();

        // Verify coset roots are shifted: coset_root[i] = root[i] * offset
        for (r, cr) in roots.iter().zip(coset_roots.iter()) {
             assert_eq!(*cr, *r * offset);
        }
    }
}
