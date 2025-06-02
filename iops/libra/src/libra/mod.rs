//! An implementation of the Libra protocol.
use circuits::layered_circuit::{LayeredCircuit, primitives::Evaluation};
use p3_field::{ExtensionField, Field};
use primitives::LibraProof;
pub mod primitives;

/// Interface for the Libra protocol.
pub trait LibraTr<F: Field, E: ExtensionField<F>> {
    fn prove(circuit: &LayeredCircuit, output: Evaluation<F>) -> LibraProof<F, E>;
    fn verify(
        circuit: &LayeredCircuit,
        proofs: LibraProof<F, E>,
        input: Vec<F>,
    ) -> Result<bool, anyhow::Error>;
}
