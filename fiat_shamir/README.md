# Fiat Shamir Transcirpt
The Fiat-Shamir transcript is a cryptographic technique used to transform an interactive proof system into a non-interactive one. This transformation enhances the efficiency and practicality of certain cryptographic protocols, including zero-knowledge proofs, by eliminating the need for interaction between the prover and the verifier.


### TranscriptInterface 

The Fiat Shamir transcript would have these two functions;
1. `append`: This is used by the prover to simulate the prover sending a message to the verifier. This function only appends `Bytes` as all these can be converted to and from `Bytes` 
2. `sample`: This is used by the prover to simulate a random reply from the verifier, this is powered by a `Random Oracle Model` using a hash function as the source of `Pseudo` randomness. would be making use of the `sha3` crate for the source of random.