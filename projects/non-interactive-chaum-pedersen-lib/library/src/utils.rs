use num::One;
use num_bigint::BigUint;
use rand::Rng;
use sha256::digest;









pub fn hash_str(message: &str) -> String {
    digest(message)
}


pub fn exponentiate(num: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    num.modpow(exp,p)
}


pub fn generate_random_32_bytes() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    BigUint::from_bytes_be(&bytes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_function() {

        let hash_one = hash_str("hello");
        let hash_two = hash_str("hello");
        let hash_three = hash_str("developeruche");

        assert_eq!(hash_one, hash_two);
        assert_ne!(hash_one, hash_three);
    }

    #[test]
    fn test_expo() {
        let num_ = BigUint::from(3u32);
        let exp_ = BigUint::from(300u32);
        let p_ = BigUint::from(10009u32);
        let ex_res = exponentiate(&num_, &exp_, &p_);

        assert_eq!(ex_res, BigUint::from(6419u32));
    }


    #[test]
    fn test_randomness() {
        let rand_one = generate_random_32_bytes();
        let rand_two = generate_random_32_bytes();


        assert_ne!(rand_one, rand_two);
    }
}