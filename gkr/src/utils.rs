use ark_ff::PrimeField;
use polynomial::multilinear::Multilinear;



/// This function is used to convert a vector of usize to a multilinear extension
/// param vec: This is the vector of usize
/// param max_size: This is the maximum size of the multilinear extension
pub fn usize_vec_to_mle<F: PrimeField>(
    vec: &[usize],
    max_size: usize,
) -> Multilinear<F> {
    let mut mle = Multilinear::zero(max_size);
    
    for i in vec {
        mle.evaluations[*i] = F::one();
    }
    
    mle
}





#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::Fr;
    use polynomial::interface::MultilinearPolynomialInterface;
    
    
    #[test]
    fn test_usize_vec_to_mle() {
        let vec = vec![1];
        let max_size = 3;
        
        let mle = usize_vec_to_mle::<Fr>(&vec, max_size);
        
        let eval_0 = mle.evaluate(&vec![Fr::from(0u32), Fr::from(0u32), Fr::from(0u32)]);
        let eval_1 = mle.evaluate(&vec![Fr::from(0u32), Fr::from(0u32), Fr::from(1u32)]);
        let eval_2 = mle.evaluate(&vec![Fr::from(1u32), Fr::from(0u32), Fr::from(0u32)]);
        
        
        // assert all binanry inputs to mle returns ZERO
        assert_eq!(eval_0, Some(Fr::from(0u32)));
        assert_eq!(eval_2, Some(Fr::from(0u32)));
        
        // assert the input to mle returns ONE
        assert_eq!(eval_1, Some(Fr::from(1u32)));
    }
    
    
    #[test]
    fn test_usize_vec_to_mle_0() {
        let vec = vec![1];
        let max_size = 5;
        
        let mle = usize_vec_to_mle::<Fr>(&vec, max_size);
        
        let eval_0_1 = mle.evaluate(&vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]);
        let eval_0_2 = mle.evaluate(&vec![Fr::from(0), Fr::from(1), Fr::from(0), Fr::from(0), Fr::from(0)]);
        let eval_0_3 = mle.evaluate(&vec![Fr::from(0), Fr::from(1), Fr::from(1), Fr::from(0), Fr::from(0)]);
        let eval_0_4 = mle.evaluate(&vec![Fr::from(1), Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]);
        let eval_0_5 = mle.evaluate(&vec![Fr::from(1), Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]);
        let eval_0_6 = mle.evaluate(&vec![Fr::from(0), Fr::from(0), Fr::from(1), Fr::from(0), Fr::from(0)]);
        
        
        let eval_1 = mle.evaluate(&vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(1)]);
        
        
        // assert all binanry inputs to mle returns ZERO
        assert_eq!(eval_0_1, Some(Fr::from(0u32)));
        assert_eq!(eval_0_2, Some(Fr::from(0u32)));
        assert_eq!(eval_0_3, Some(Fr::from(0u32)));
        assert_eq!(eval_0_4, Some(Fr::from(0u32)));
        assert_eq!(eval_0_5, Some(Fr::from(0u32)));
        assert_eq!(eval_0_6, Some(Fr::from(0u32)));
        
        
        // assert the input to mle returns ONE
        assert_eq!(eval_1, Some(Fr::from(1u32)));
    }
}