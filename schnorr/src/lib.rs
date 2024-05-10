pub mod core;


/// This function is used to sign a message over a generic elliptic curve and generic hash function.
fn sign() {

}

/// This function is used for verifying a signature over a message
fn verify() {

}

/// This function is used to generate a new key pair
fn new_key_pair() {

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
