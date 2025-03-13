# ECDSA Implementation is RUST
--------------------------------

This library provides an implementation of the Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust. It allows you to:

- Generate ECDSA keypairs (private and public keys)
- Sign messages using a private key
- Verify signatures using a public key


### Little Technical details

1. This `Sign` function:
	```
	R = k A -> take `r = x` component
	s = (hash(message) + d * r) * k^(-1) mod q
	```
2. The `Verify` function:
	Verifies if a signature is valid for a particular message hash and public key.
  ```
  	(s, r) = signature
      u1 = s^(-1) * hash(message) mod q
      u2 = s^(-1) * r mod q
      P = u1 A + u2 B mod q = (xp, yp)
      if r == xp then verified!
  ```

### Pseudo code

```rust 
    let elliptic_curve = EllipticCurve {
        a: BigUint::from(2u32),
        b: BigUint::from(2u32),
        p: BigUint::from(17u32),
    };

    let a_gen = CurvePoint::Coordinate(BigUint::from(5u32),BigUint::from(1u32));

    let q_order = BigUint::from(19u32);

    let ecdsa = ECDSA {
        elliptic_curve,
        a_gen,
        q_order,
    };

    let priv_key = BigUint::from(7u32);
    let pub_key = ecdsa
        .generate_pub_key(&priv_key)
        .expect("Could not compute PubKey");

    let k_random = BigUint::from(18u32);

    let message = "Bob -> 1 BTC -> Alice";
    let hash = ecdsa.generate_hash_less_than(message, &ecdsa.q_order);

    let signature = ecdsa
        .sign(&hash, &priv_key, &k_random)
        .expect("Could not sign");

    let verify_result = ecdsa
        .verify(&hash, &pub_key, &signature)
        .expect("Could not verify");

    assert!(verify_result, "Verification should success");
  ```
