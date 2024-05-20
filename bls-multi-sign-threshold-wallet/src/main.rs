use polynomial::{
    ark_ff::Field,
    ark_test_curves::bls12_381::Fr,
    interface::{PolynomialInterface, UnivariantPolynomialInterface},
    univariant::UnivariantPolynomial,
};
// use polynomial::univariant::

fn main() {
    //testing the shamir secret sharing
    let threshold = 3;
    let members = 5;

    let secret = Fr::from(1234567890u32);

    let (shares_x, shares_y) = shamir_secret_sharing(threshold, members, secret);

    let user_x = shares_x[1..].to_vec();
    let user_y = shares_y[1..].to_vec();

    println!("Recovering with {} - and - {}", user_x.len(), user_y.len());

    let recovered_secret = recover_secret(user_x, user_y);

    println!("Recovered secret: {:?}", recovered_secret);

    assert!(secret == recovered_secret);
}

pub fn shamir_secret_sharing<F: Field>(
    threshold: usize,
    members: usize,
    secret: F,
) -> (Vec<F>, Vec<F>) {
    let mut rng = rand::thread_rng();
    let mut domain = Vec::with_capacity(threshold + 1);
    let mut y_s = Vec::with_capacity(threshold + 1);

    for i in 0..threshold {
        domain.push(F::from(i as u32));
    }

    for _ in 0..threshold {
        y_s.push(F::rand(&mut rng));
    }

    y_s[0] = secret;

    let poly = UnivariantPolynomial::interpolate(y_s, domain);

    if poly.degree() != threshold - 1 {
        panic!("Polynomial degree is not correct");
    }

    let mut shares_y = Vec::with_capacity(members);
    let mut shares_x = Vec::with_capacity(members);

    for i in 0..members {
        let x = F::from(i as u32);
        let y = poly.evaluate(&x);
        shares_x.push(x);
        shares_y.push(y);
    }

    (shares_x, shares_y)
}

pub fn recover_secret<F: Field>(shares_x: Vec<F>, shares_y: Vec<F>) -> F {
    let poly = UnivariantPolynomial::interpolate(shares_y.clone(), shares_x.clone());
    let secret = poly.evaluate(&F::zero());
    secret
}
