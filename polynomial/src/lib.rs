use ark_poly::{univariate::DensePolynomial, Evaluations, Polynomial};
pub mod interface;
pub mod univariant;

pub fn add(left: usize, right: usize) -> usize {

    // let poly = Evaluations;
    left + right
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
