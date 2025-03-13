use ark_circom::{CircomBuilder, CircomConfig};
use ark_std::rand::thread_rng;
use ark_bn254::Bn254;


type Groth16Protocol = groth16::protocol::Groth16Protocol<Bn254>;

fn main() {
    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(
        "./assets/mycircuit.wasm",
        "./assets/mycircuit.r1cs",
    ).unwrap();
    
    // Insert our public inputs as key value pairs
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);
    
    // Create an empty instance for setting it up
    let circom = builder.setup();
    
    // Run a trusted setup
    let mut rng = thread_rng();
}