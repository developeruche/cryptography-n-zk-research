The GKR protocol is very fascinating, fairly not so complicated compared to other protocols but heavily generic and useful. The GKR protocol involves running one protocol (Sum check) inside this protocol (GKR). The GKR protocol, named after Shafi Goldwasser, Yael Tauman Kalai, and Guy N. Rothblum, is a zero-knowledge proof protocol that focuses on efficient delegation of computation. Specifically, it is designed to verify computations represented as layered arithmetic circuits with logarithmic depth in the number of inputs. The GKR protocol is known for its efficiency and ability to handle large-scale computations. 

This implemenation has two variants;
1. Normal GKR (with Fiat Shamir heuristic)
2. Succint GKR Using  Multilinear-KZG polynomial commitments


### Features
	-	Efficient Prover: Supports the GKR proof construction with optimized arithmetic circuits.
	-	Verifier: Verifies the integrity of the GKR proof using sum-check protocol rounds.
	-	Polynomial Commitment Support: (Succinct GKR variant) Uses Multilinear-KZG commitments for efficient verification.
	- Circuit Support: Supports general arithmetic circuits with addition and multiplication gates.
	- Succinct Prover: Implements a succinct variant of the GKR protocol with multilinear-KZG polynomial commitments.
	
	
### Usage

#### Normal GKR Protocol
```rust
```rust 
fn test_gkr_protocol() {
    // Define the circuit and input
    let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
    // More circuit setup...
    
    // Evaluate the circuit
    let evaluation = circuit.evaluate(&input);

    // Generate the GKR proof
    let proof = GKRProtocol::prove(&circuit, &evaluation);

    // Verify the proof
    assert!(GKRProtocol::verify(&circuit, &input, &proof));
}
```

#### Succinct GKR Protocol
```rust
use succinct_gkr::SuccinctGKRProtocol;
use circuits::{Circuit, CircuitLayer, Gate, GateType};
use ark_test_curves::bls12_381::{Bls12_381, Fr};
use polynomial::multilinear::Multilinear;

fn main() {
    // Define a sample circuit
    let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
    let layer_1 = CircuitLayer::new(vec![
        Gate::new(GateType::Mul, [0, 1]),
        Gate::new(GateType::Add, [2, 3]),
    ]);

    let circuit = Circuit::new(vec![layer_0, layer_1]);
    let input = vec![Fr::from(2u32), Fr::from(1u32), Fr::from(3u32), Fr::from(1u32)];
    
    // Convert input into polynomial form
    let input_poly = Multilinear::interpolate(&input);

    // Generate an SRS for KZG commitments
    let tau_s = succinct_gkr::utils::gen_random_taus::<Fr>(input_poly.num_vars);
    let srs = succinct_gkr::MultilinearKZG::<Bls12_381>::generate_srs(&tau_s);

    // Commit to the input polynomial
    let commitment = succinct_gkr::MultilinearKZG::commit(&srs, &input_poly);

    // Evaluate the circuit
    let evaluation = circuit.evaluate(&input);

    // Generate a succinct GKR proof
    let proof = SuccinctGKRProtocol::prove(&circuit, &evaluation, &commitment, &srs);

    // Verify the proof
    let is_valid = SuccinctGKRProtocol::verify(&circuit, &proof, &commitment, &srs);
    assert!(is_valid);
}
```


License

This project is licensed under the MIT License.
