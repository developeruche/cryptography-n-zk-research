//! The goal of this module is to provide a way to transpile a circuit into a Plonkish representation.
use crate::primitives::{Circuit, GateType};


pub fn plonkish_transpile(circuit: &Circuit) -> Vec<String> {
    let var_prefix = "v";
    let mut asq = Vec::new();

    for (i, layer) in circuit.layers.iter().enumerate() {
        for (i_g, gate) in layer.layer.iter().enumerate() {
            match gate.g_type {
                GateType::Add => {
                    // example: "v01 <== v10 + v11"
                    let as_ = format!(
                        "{} <== {} + {}",
                        format!("{}{}{}", var_prefix, i, i_g),
                        format!("{}{}{}", var_prefix, i + 1, i_g * 2),
                        format!("{}{}{}", var_prefix, i + 1, (i_g * 2) + 1)
                    );
                    asq.push(as_);
                }
                GateType::Mul => {
                    let as_ = format!(
                        "{} <== {} * {}",
                        format!("{}{}{}", var_prefix, i, i_g),
                        format!("{}{}{}", var_prefix, i + 1, i_g * 2),
                        format!("{}{}{}", var_prefix, i + 1, (i_g * 2) + 1)
                    );
                    asq.push(as_);
                }
            }
        }
    }

    asq
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{CircuitLayer, Gate};
    
    
    #[test]
    fn test_plonkish_transpile() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        
        let asq = plonkish_transpile(&circuit);
        assert_eq!(asq, vec!["v00 <== v10 * v11".to_string(), "v10 <== v20 + v21".to_string(), "v11 <== v22 * v23".to_string()])
    }
}
