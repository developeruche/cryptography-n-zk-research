# Non-Interactive Chaum Pedersen Lib
---------------------------------------

The Chaum-Pedersen Zero-Knowledge Proof (ZKP) protocol allows a prover to convince a verifier that they possess a secret value (knowledge) without revealing the actual value itself. It operates in an interactive setting, meaning the prover and verifier exchange messages back and forth.

## Non-Interactive Implementation
The standard Chaum-Pedersen protocol is interactive, but it can be converted into a non-interactive version using the Fiat-Shamir heuristic. This transformation removes the need for real-time interaction:



_ TLDR: Non-technical introduction, skip this if you don't need this _

Prover Setup:

The prover generates a secret value (x) they want to keep hidden.
They choose system parameters, which include a large prime number (p) and two generators (g and h) within a group modulo p.
The prover computes commitments to their secret (y1 = g^x mod p and y2 = h^x mod p).
They generate a random value (k) and create blinding factors (r1 = g^k mod p and r2 = h^k mod p).
Challenge Generation (Non-Interactive):

Unlike the interactive version where the verifier sends a challenge, the prover now generates a challenge themselves.
They use a secure hash function (H) to combine the commitments (y1, y2), blinding factors (r1, r2), and potentially other public information to create a challenge (c = H(y1, y2, r1, r2, ...)).
Response Calculation:

The prover calculates a response (s) using their secret (x), random value (k), challenge (c), and system parameter (p). This response depends on the specific variant of the protocol (e.g., s = k - cx mod (p - 1)).
Verification:

The verifier receives the commitments (y1, y2), blinding factors (r1, r2), response (s), and system parameters (g, h, p).
They recompute the challenge (c' = H(y1, y2, r1, r2, ...)).
They verify the proof by checking if the following equation holds: g^s * y1^(c') = r1 mod p and h^s * y2^(c') = r2 mod p. If both equations hold, the verifier is convinced that the prover knows the secret x without learning its exact value.
Benefits of Non-Interactive Implementation:

Reduced Communication: No real-time interaction is required between the prover and verifier, making it suitable for asynchronous communication scenarios.
Improved Efficiency: Eliminates the back-and-forth messages, potentially leading to faster verification.
Drawbacks:

Security Relies on Hash Function: The security of the non-interactive version depends on the collision resistance of the hash function H. A weak hash function could allow for forgery of proofs.
Standardization: Non-interactive ZKPs are generally less standardized than interactive ones, potentially leading to compatibility issues.
Overall, the non-interactive Chaum-Pedersen ZKP protocol offers a convenient way for a prover to demonstrate knowledge of a secret value without revealing it, especially in situations where real-time communication is impractical.



Features:
- Efficient implementation of the non-interactive Chaum-Pedersen protocol
- Support for both prime field and elliptic curve groups (planned)
- Clear and concise API
- Unit tests


```rust

use library::*;
use library::utils::{exponentiate, generate_random_32_bytes};


fn main() {
    let system_default = NICP::new();


    let x = BigUint::from(300u32);
    let y1 = exponentiate(&system_default.alpha, &x, &system_default.modulus);
    let y2 = exponentiate(&system_default.beta, &x, &system_default.modulus);

    let k = BigUint::from(10u32);

    let r1 = exponentiate(&system_default.alpha, &k, &system_default.modulus);
    let r2 = exponentiate(&system_default.beta, &k, &system_default.modulus);

    let c = gen_challenge(
        &y1,
        &y2,
        &r1,
        &r2
    );


    let solution = solve_challenge(
        &k,
        &x,
        &c,
        &system_default.order
    );


    let verify = verify_challenge(&system_default.alpha, &system_default.beta, &solution, &c, &y1, &y2, &system_default.modulus);



    dbg!("==============================");
    println!(" Here come the verification:::------->   {}", verify);
    dbg!("==============================");
}

```


### API Reference

NICP:
- new: Creates a new instance of NICP with default system parameters. (Note: This currently uses hardcoded values. In future versions, consider allowing customization.)
utils:
- exponentiate: Performs modular exponentiation.
- generate_random_32_bytes: Generates a random 32-byte value (intended for elliptic curve implementations).
- gen_challenge: Generates a challenge based on commitments and random values.
- solve_challenge: Calculates the response to the challenge.
- verify_challenge: Verifies the proof using the commitments, challenge, response, and system parameters.
  
### Security Considerations

This library is currently in its early stages and may not be suitable for production use. Further security analysis is recommended.
Elliptic curve support is planned for improved security properties.
Consider using secure random number generation for generate_random_32_bytes.
Contributing

We welcome contributions to this project! Please refer to the CONTRIBUTING.md file (if available) for guidelines.

License

This library is licensed under the MIT License (see LICENSE file).
