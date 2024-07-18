use std::collections::HashMap;
use crate::{
    interfaces::ExtractConstraintsInterface,
    primitives::{Circuit, Constraint, ConstraintRaw, ConstraintsWithLabelSize, GateType},
    utils::compute_constraint_item,
};


impl ExtractConstraintsInterface for Circuit {
    fn extract_constraints(&self) -> ConstraintsWithLabelSize {
        let mut label_to_index_mapping = HashMap::new();
        let mut latest_constraint_index = 1;
        let mut raw_constraints = Vec::<ConstraintRaw>::new();

        for (layer_index, layer) in self.layers.iter().enumerate() {
            for (gate_index, gate) in layer.layer.iter().enumerate() {
                let c_label = compute_constraint_item(layer_index, gate_index);
                let a_label = compute_constraint_item(layer_index + 1, gate.inputs[0]);
                let b_label = compute_constraint_item(layer_index + 1, gate.inputs[1]);

                match label_to_index_mapping.entry(c_label) {
                    std::collections::hash_map::Entry::Occupied(_) => {},
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(latest_constraint_index);
                        latest_constraint_index += 1;
                    },
                }
                
                match label_to_index_mapping.entry(a_label) {
                    std::collections::hash_map::Entry::Occupied(_) => {},
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(latest_constraint_index);
                        latest_constraint_index += 1;
                    },
                }
                
                match label_to_index_mapping.entry(b_label) {
                    std::collections::hash_map::Entry::Occupied(_) => {},
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(latest_constraint_index);
                        latest_constraint_index += 1;
                    },
                }

                match gate.g_type {
                    GateType::Add => {
                        let labeled_constraint = ConstraintRaw {
                            a: vec![a_label, b_label],
                            b: vec![],
                            c: vec![c_label],
                        };

                        raw_constraints.push(labeled_constraint);
                    }
                    GateType::Mul => {
                        let labeled_constraint = ConstraintRaw {
                            a: vec![a_label],
                            b: vec![b_label],
                            c: vec![c_label],
                        };

                        raw_constraints.push(labeled_constraint);
                    }
                }
            }
        }

        let constraints = raw_constraints
            .iter()
            .map(|raw_constraint| raw_constraint.to_constraint(label_to_index_mapping.clone()))
            .collect::<Vec<Constraint>>();

        ConstraintsWithLabelSize {
            constraints,
            label_size: latest_constraint_index,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::primitives::{CircuitLayer, Gate, Witness};
    use ark_test_curves::bls12_381::Fr;
    use super::*;
    

    // sample circuit evaluation
    //      100(*)    - layer 0
    //     /     \
    //   5(+)_0   20(*)_1 - layer 1
    //   / \    /  \
    //  2   3   4   5
    #[test]
    fn test_circuit_to_r1cs() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 1]),
            Gate::new(GateType::Add, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        
        let constraints = circuit.extract_constraints();
        
        assert_eq!(constraints.label_size, 8);
        assert_eq!(constraints.constraints.len(), 3);
        
        let r1cs = constraints.to_r1cs_vec::<Fr>();
        assert_eq!(r1cs.a.len(), 3);
        assert_eq!(r1cs.b.len(), 3);
        assert_eq!(r1cs.c.len(), 3);
    }
    
    
    
    // sample circuit evaluation
    //      100(*)    - layer 0
    //     /     \
    //   5(+)_0   20(*)_1 - layer 1
    //   / \    /  \
    //  2   3   4   5
    #[test]
    fn test_circuit_to_r1cs_with_checks() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 1]),
            Gate::new(GateType::Add, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        
        let constraints = circuit.extract_constraints();
        
        assert_eq!(constraints.label_size, 8);
        assert_eq!(constraints.constraints.len(), 3);
        
        let r1cs = constraints.to_r1cs_vec::<Fr>();
        
        let witness = Witness::new(
            vec![Fr::from(1u32)],
            vec![
                Fr::from(15u32),
                Fr::from(6u32),
                Fr::from(9u32),
                Fr::from(2u32),
                Fr::from(3u32),
                Fr::from(4u32),
                Fr::from(5u32),
            ],
        );
        
        println!("r1cs a: {:?}", r1cs.c);
        
        let r1cs_check = r1cs.check(witness.render());
        assert!(r1cs_check, "this is the R1CS check");
    }
}