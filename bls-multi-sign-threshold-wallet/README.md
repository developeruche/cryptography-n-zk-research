# Shamir Secret Sharing in Rust

This project implements Shamir’s Secret Sharing scheme using Rust. It allows a secret to be split into multiple shares, such that only a specified threshold number of shares is required to reconstruct the secret. This implementation uses polynomials for secret sharing and secret recovery.

## Overview

Shamir’s Secret Sharing is a cryptographic algorithm used to secure a secret among multiple participants. The secret is divided into parts, and a minimum number of parts are required to reconstruct the original secret. In this implementation:

	•	You can specify a threshold number of participants required to recover the secret.
	•	The secret is encoded into a polynomial, and the shares are points on this polynomial.
	•	The secret can only be recovered if at least the threshold number of shares are available.

## Features

	•	Threshold-based sharing: Define how many participants are needed to reconstruct the secret.
	•	Secret sharing: Distribute the secret into multiple shares.
	•	Secret recovery: Reconstruct the original secret if enough shares are provided.

Dependencies

This project uses the following Rust crates:

	•	ark-ff: Finite field arithmetic.
	•	ark-test-curves: Elliptic curves, specifically bls12-381.
	•	polynomial: Provides the polynomial arithmetic and evaluation tools.

Make sure to include these dependencies in your Cargo.toml.

```toml 
    [dependencies]
    polynomial = {path = "../polynomial"}
    rand = "0.8.5"
```

## Usage

To test the Shamir secret sharing implementation:

	1.	Set the threshold (minimum number of shares needed) and the total number of participants.
	2.	Define the secret to be shared.
	3.	Run the program to generate shares, then recover the secret using the generated shares.
	
```rust
    let threshold = 3;
    let members = 5;
    let secret = Fr::from(1234567890u32);
    
    let (shares_x, shares_y) = shamir_secret_sharing(threshold, members, secret);
    
    let user_x = shares_x[1..].to_vec();
    let user_y = shares_y[1..].to_vec();
    
    let recovered_secret = recover_secret(user_x, user_y);
```

In this example:

	•	The secret is shared among 5 participants, and a threshold of 3 shares is required to recover the secret.
	•	The secret is successfully recovered using 4 shares in this case.
	
## Functions

	•	shamir_secret_sharing(threshold: usize, members: usize, secret: F): Splits a secret into members shares, where a minimum of threshold shares is required to recover the secret.
	•	recover_secret(shares_x: Vec<F>, shares_y: Vec<F>): Recovers the secret using the given x and y coordinates (shares).
	
## Running the Code

To run the code, use:

```bash
    cargo run
```
This will execute the example provided in main().

## License
This project is licensed under the MIT License. See the LICENSE file for more details.