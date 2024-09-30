# Fiat Shamir Transcirpt
The Fiat-Shamir transcript is a cryptographic technique used to transform an interactive proof system into a non-interactive one. This transformation enhances the efficiency and practicality of certain cryptographic protocols, including zero-knowledge proofs, by eliminating the need for interaction between the prover and the verifier.


### TranscriptInterface 

The Fiat Shamir transcript would have these two functions;
1. `append`: This is used by the prover to simulate the prover sending a message to the verifier. This function only appends `Bytes` as all these can be converted to and from `Bytes` 
2. `sample`: This is used by the prover to simulate a random reply from the verifier, this is powered by a `Random Oracle Model` using a hash function as the source of `Pseudo` randomness. would be making use of the `sha3` crate for the source of random.


### Usage

This module implements a Fiat-Shamir heuristic transcript using the Keccak256 hash function. The FiatShamirTranscript struct serves as the transcript and provides methods to append messages, sample challenges, and convert them into field elements.

**Creating a Transcript**

To create a new transcript, instantiate the FiatShamirTranscript with an initial message using the new method.
```rust
use sha3::Keccak256;
use your_crate::FiatShamirTranscript;

let message = vec![1, 2, 3, 4];
let mut transcript = FiatShamirTranscript::new(message);
```

**Appending Messages to the Transcript**

You can append additional messages to the transcript using the append method. This updates the internal state of the transcript.

```rust
let new_message = vec![5, 6, 7, 8];
transcript.append(new_message);
```

**Sampling a Challenge**

The transcript can sample random challenges based on the current state. Use the sample method to get a 32-byte challenge.

```rust
let challenge = transcript.sample();
```

You can also generate multiple challenges at once using the `sample_n` method.

```rust

let challenges = transcript.sample_n(5);
```

**Sampling Field Elements**

To sample challenges as field elements in a prime field, use the sample_as_field_element method. This requires the field type F to be specified, where F implements ark_ff::PrimeField.

```rust 
use ark_ff::fields::PrimeField;
use ark_test_curves::bls12_381::Fr; // Example field element

let field_element: Fr = transcript.sample_as_field_element::<Fr>();
```

For generating multiple field elements, use sample_n_as_field_elements.

```rust
let field_elements: Vec<Fr> = transcript.sample_n_as_field_elements::<Fr>(5);
```

### Complete Example

```rust
fn main() {
    let message = vec![1, 2, 3, 4];
    let mut transcript = FiatShamirTranscript::new(message);

    // Append additional data
    transcript.append(vec![5, 6, 7, 8]);

    // Sample a challenge
    let challenge = transcript.sample();
    println!("Challenge: {:?}", challenge);

    // Sample a field element
    let field_element: Fr = transcript.sample_as_field_element::<Fr>();
    println!("Field Element: {:?}", field_element);

    // Sample multiple field elements
    let field_elements: Vec<Fr> = transcript.sample_n_as_field_elements::<Fr>(3);
    println!("Field Elements: {:?}", field_elements);
}
```