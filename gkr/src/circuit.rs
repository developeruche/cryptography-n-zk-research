use crate::{
    interfaces::CircuitInterface,
    primitives::{Circuit, CircuitEvaluation, GateType},
};
use std::ops::{Add, Mul};

impl CircuitInterface for Circuit {
    fn evaluate<F>(&self, input: &[F]) -> CircuitEvaluation<F>
    where
        F: Add<Output = F> + Mul<Output = F> + Copy,
    {
        let mut layers = vec![];
        let mut current_input = input;
        layers.push(input.to_vec());

        for layer in self.layers.iter().rev() {
            let temp_layer: Vec<_> = layer
                .layer
                .iter()
                .map(|e| match e.g_type {
                    GateType::Add => current_input[e.inputs[0]] + current_input[e.inputs[1]],
                    GateType::Mul => current_input[e.inputs[0]] * current_input[e.inputs[1]],
                })
                .collect();
            layers.push(temp_layer);
            current_input = &layers[layers.len() - 1];
        }

        layers.reverse();
        CircuitEvaluation { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{CircuitLayer, Gate};
    use ark_test_curves::bls12_381::Fr;

    // sample circuit evaluation
    //      100(*)    - layer 0
    //     /     \
    //   5(+)_0   20(*)_1 - layer 1
    //   / \    /  \
    //  2   3   4   5
    //
    #[test]
    fn test_circuit_evaluation_1() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Mul, [0, 1])]);
        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);
        let circuit = Circuit::new(vec![layer_0, layer_1]);
        let input = [
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(4u32),
            Fr::from(5u32),
        ];
        let evaluation = circuit.evaluate(&input);
        let expected_output = vec![
            vec![Fr::from(100u32)],
            vec![Fr::from(5u32), Fr::from(20u32)],
            vec![
                Fr::from(2u32),
                Fr::from(3u32),
                Fr::from(4u32),
                Fr::from(5u32),
            ],
        ];

        assert_eq!(evaluation.layers, expected_output);
    }

    // Circuit is hard to draw :)
    #[test]
    fn test_circuit_evaluation_2() {
        let layer_0 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);

        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Mul, [0, 0]),
            Gate::new(GateType::Mul, [1, 1]),
            Gate::new(GateType::Mul, [1, 2]),
            Gate::new(GateType::Mul, [3, 3]),
        ]);

        let circuit = Circuit::new(vec![layer_0, layer_1]);
        let evaluation = circuit.evaluate(&[
            Fr::from(3u32),
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(1u32),
        ]);

        let expected_output = vec![
            vec![Fr::from(36u32), Fr::from(6u32)],
            vec![
                Fr::from(9u32),
                Fr::from(4u32),
                Fr::from(6u32),
                Fr::from(1u32),
            ],
            vec![
                Fr::from(3u32),
                Fr::from(2u32),
                Fr::from(3u32),
                Fr::from(1u32),
            ],
        ];

        assert_eq!(evaluation.layers, expected_output);
    }

    #[test]
    fn test_circuit_evaluation_3() {
        let layer_0 = CircuitLayer::new(vec![Gate::new(GateType::Add, [0, 1])]);

        let layer_1 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
        ]);

        let layer_2 = CircuitLayer::new(vec![
            Gate::new(GateType::Add, [0, 1]),
            Gate::new(GateType::Mul, [2, 3]),
            Gate::new(GateType::Mul, [4, 5]),
            Gate::new(GateType::Mul, [6, 7]),
        ]);

        let circuit = Circuit::new(vec![layer_0, layer_1, layer_2]);

        let evaluation = circuit.evaluate(&[
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(1u32),
            Fr::from(4u32),
            Fr::from(1u32),
            Fr::from(2u32),
            Fr::from(3u32),
            Fr::from(4u32),
        ]);

        let expected_output = vec![
            vec![Fr::from(33u32)],
            vec![Fr::from(9u32), Fr::from(24u32)],
            vec![
                Fr::from(5u32),
                Fr::from(4u32),
                Fr::from(2u32),
                Fr::from(12u32),
            ],
            vec![
                Fr::from(2u32),
                Fr::from(3u32),
                Fr::from(1u32),
                Fr::from(4u32),
                Fr::from(1u32),
                Fr::from(2u32),
                Fr::from(3u32),
                Fr::from(4u32),
            ],
        ];

        assert_eq!(evaluation.layers, expected_output);
    }
}
