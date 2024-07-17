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
        let mut raw_constraints = Vec::new();

        for (layer_index, layer) in self.layers.iter().enumerate() {
            for (gate_index, gate) in layer.layer.iter().enumerate() {
                let c_label = compute_constraint_item(layer_index, gate_index);
                let a_label = compute_constraint_item(layer_index + 1, gate.inputs[0]);
                let b_label = compute_constraint_item(layer_index + 1, gate.inputs[1]);

                label_to_index_mapping.insert(c_label, latest_constraint_index);
                latest_constraint_index += 1;
                label_to_index_mapping.insert(a_label, latest_constraint_index);
                latest_constraint_index += 1;
                label_to_index_mapping.insert(b_label, latest_constraint_index);
                latest_constraint_index += 1;

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
