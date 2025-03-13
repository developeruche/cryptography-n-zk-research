///
/// A library which implements the bottom layer finite field group operations needed to
/// operate with the coordinates of the elliptic curve group.
///
use num_bigint::BigUint;




#[derive(Debug, PartialEq)]
pub enum FiniteFieldError {
    InvalidArgument(String),
    InvalidResult(String),
}


///
/// Adds to elements in the set (this would also return a point in the curve)
///
/// `a + b = a mod p`
///

pub fn add(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;

    Ok((a + b).modpow(&BigUint::from(1u32), p))
}



///
/// Multiplies to elements in the set
///
/// `a * b = a mod p`
///
pub fn multiplicate(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;

    Ok((a * b).modpow(&BigUint::from(1u32), p))
}


///
/// Finds the additive inverse of an element in the set:
///
/// `a + (-a) = 0 mod p`
///
pub fn inverse_add(a: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check_single_point(a,p)?;
    if *a == BigUint::from(0u32) {
        return Ok(a.clone());
    }

    Ok(p - a)
}


///
/// Subtract two elements in the set:
///
/// `a - b = a + (-b) = a mod p`
///
pub fn subtract(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;
    let b_inverse = inverse_add(b, p)?;

    add(a, &b_inverse, p)
}


///
/// Finds the multiplicative inverse of an element in the set if p is a
/// prime number using Fermat's Little Theorem:
///
/// `a^(-1) mod p = a^(p-2) mod p`
///
/// Such that:
/// `a * a^(-1) = 1 mod p`
///
pub fn inverse_multiplicate_prime(a: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check_single_point(a, p)?;
    Ok(a.modpow(&(p - BigUint::from(2u32)), p))
}


///
/// Divides two elements in the set:
///
/// `a / b = a * b^(-1) = a mod p`
///
pub fn divide(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<BigUint, FiniteFieldError> {
    params_to_mod_check(a,b,p)?;
    let b_inverse = inverse_multiplicate_prime(b, p)?;

    multiplicate(a, &b_inverse, p)
}












///
/// This function check if `a  < b`; if a is b, function would return true
///
pub fn check_is_less_than(a: &BigUint, b: &BigUint)  -> bool {
    if a < b {
        true
    } else {
        false
    }
}

///
/// When adding elliptic curves point, it is important that the points to be added id not
/// greater than the operating mob p of the group.
pub fn params_to_mod_check(a: &BigUint, b: &BigUint, p: &BigUint) -> Result<(), FiniteFieldError> {
    let params_check = check_is_less_than(a, p) && check_is_less_than(b, p);
    if !params_check {
        return Err(FiniteFieldError::InvalidArgument(format!("a and b has to be greater than p: {}, {}, {}", a, b, p)))
    }

    Ok(())
}

pub fn params_to_mod_check_single_point(a: &BigUint, p: &BigUint) -> Result<(), FiniteFieldError> {
    let params_check = check_is_less_than(a, p);
    if !params_check {
        return Err(FiniteFieldError::InvalidArgument(format!("a and b has to be greater than p: {}, {}", a, p)))
    }

    Ok(())
}

























// ===================================
// TEST ------------------------------
// ===================================


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let res = add(&a, &b, &p).unwrap();
        assert_eq!(res, BigUint::from(3u32));

        let a = BigUint::from(10u32);
        let b = BigUint::from(1u32);
        let p = BigUint::from(11u32);

        let res = add(&a, &b, &p).unwrap();
        assert_eq!(res, BigUint::from(0u32));

        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(31u32);

        let res = add(&a, &b, &p).unwrap();
        assert_eq!(res, BigUint::from(14u32));
    }

    #[test]
    fn test_multiply() {
        let a = BigUint::from(4u32);
        let b = BigUint::from(10u32);
        let p = BigUint::from(11u32);

        let res = multiplicate(&a, &b, &p).unwrap();
        assert_eq!(res, BigUint::from(7u32));

        let p = BigUint::from(51u32);

        let res = multiplicate(&a, &b, &p).unwrap();
        assert_eq!(res, BigUint::from(40u32));
    }

    #[test]
    fn test_inv_add() {
        let a = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let res = inverse_add(&a, &p).unwrap();
        assert_eq!(res, BigUint::from(47u32));

        let a = BigUint::from(0u32);
        let p = BigUint::from(51u32);

        let res = inverse_add(&a, &p).unwrap();
        assert_eq!(res, BigUint::from(0u32));

        let a = BigUint::from(52u32);
        let p = BigUint::from(51u32);

        assert_eq!(
            inverse_add(&a, &p),
            Err(FiniteFieldError::InvalidArgument(format!("a and b has to be greater than p: {}, {}", a, p)))
        );

        let a = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        let c_inv = inverse_add(&a, &p);

        assert_eq!(c_inv, Ok(BigUint::from(47u32)));
        assert_eq!(
            add(&a, &c_inv.unwrap(), &p),
            Ok(BigUint::from(0u32))
        );
    }

    #[test]
    fn test_subtract() {
        // a - a = 0 mod p
        let a = BigUint::from(4u32);
        let p = BigUint::from(51u32);

        assert_eq!(subtract(&a, &a, &p), Ok(BigUint::from(0u32)));
    }

    #[test]
    fn test_inv_mult() {
        // 4 * 3 mod 11 = 12 mod 11 = 1
        let a = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        let c_inv = inverse_multiplicate_prime(&a, &p);

        assert_eq!(c_inv, Ok(BigUint::from(3u32)));
        assert_eq!(
            multiplicate(&a, &c_inv.unwrap(), &p),
            Ok(BigUint::from(1u32))
        );
    }

    #[test]
    fn test_divide() {
        // a / a = 1 mod p
        let a = BigUint::from(4u32);
        let p = BigUint::from(11u32);

        assert_eq!(divide(&a, &a, &p), Ok(BigUint::from(1u32)));
    }
}
