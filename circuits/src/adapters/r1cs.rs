use crate::{
    interfaces::ExtractConstraintsInterface,
    primitives::{Circuit, Constraints, ConstraintsRaw},
};

impl ExtractConstraintsInterface for Circuit {
    fn extract_constraints(&self) -> Constraints {
        for (layer_index, layer) in self.layers.iter().enumerate() {
            for (gate_index, gate) in layer.layer.iter().enumerate() {
                let pin = if layer_index == 0 {
                    0
                } else {
                    let mut cache_pin = 0;

                    for i in 0..layer_index {
                        let size_of_prev_layer = self.layers[layer_index - 1].layer.len();
                    }

                    0
                };
                let label = layer.layer.len() + gate_index + 1;
                let raw_c = ConstraintsRaw {
                    input: [gate.inputs[0] + pin, gate.inputs[1] + pin],
                    gate_type: gate.g_type.clone(),
                    label,
                };
            }
        }

        todo!()
    }
}
