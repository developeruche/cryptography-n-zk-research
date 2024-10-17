#![allow(non_snake_case)]

use ark_ff::PrimeField;
use std::collections::HashMap;

use crate::utils::{get_product_key, is_valid_variable_name, merge_maps, multiply_maps};

/// Variable names for Left, Right, and Output wires.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GateWire {
    pub L: Option<String>,
    pub R: Option<String>,
    pub O: Option<String>,
}

/// This is the gate polynomial
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gate<F: PrimeField> {
    pub L: F,
    pub R: F,
    pub M: F,
    pub O: F,
    pub C: F,
}

/// Assembly equation mapping wires to coefficients.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembleEquation<F: PrimeField> {
    pub wires: GateWire,
    pub coeffs: HashMap<Option<String>, F>,
}

impl<F: PrimeField> AssembleEquation<F> {
    pub fn L(&self) -> F {
        -*self.coeffs.get(&self.wires.L).unwrap_or(&F::ZERO)
    }
    pub fn R(&self) -> F {
        if self.wires.R != self.wires.L {
            -*self.coeffs.get(&self.wires.R).unwrap_or(&F::ZERO)
        } else {
            F::zero()
        }
    }
    pub fn C(&self) -> F {
        -*self.coeffs.get(&Some("".to_string())).unwrap_or(&F::ZERO)
    }
    pub fn O(&self) -> F {
        *self
            .coeffs
            .get(&Some("$output_coeff".to_string()))
            .unwrap_or(&F::ONE)
    }
    pub fn M(&self) -> F {
        if !self.wires.as_list().contains(&None) {
            return -*self
                .coeffs
                .get(&get_product_key(self.wires.L.clone(), self.wires.R.clone()))
                .unwrap_or(&F::ZERO);
        }
        F::zero()
    }

    pub fn gate(&self) -> Gate<F> {
        Gate {
            L: self.L(),
            R: self.R(),
            M: self.M(),
            O: self.O(),
            C: self.C(),
        }
    }
}

/// Converts a arithmetic expression containing numbers, variables and {+, -, *}
/// into a mapping of term to coefficient
///
/// For example:
/// ['a', '+', 'b', '*', 'c', '*', '5'] becomes {'a': 1, 'b*c': 5}
///
/// Note that this is a recursive algo, so the input can be a mix of tokens and
/// mapping expressions
///
pub fn evaluate_expression<F: PrimeField>(
    exprs: &Vec<&str>,
    first_is_negative: bool,
) -> HashMap<Option<String>, F> {
    match exprs.iter().any(|&x| x == "+") {
        true => {
            let idx = exprs.iter().position(|&x| x == "+").unwrap();
            let l = evaluate_expression(&exprs[..idx].to_vec(), first_is_negative);
            let r = evaluate_expression(&exprs[idx + 1..].to_vec(), false);
            return merge_maps(&l, &r);
        }
        false => match exprs.iter().any(|&x| x == "-") {
            true => {
                let idx = exprs.iter().position(|&x| x == "-").unwrap();
                let l = evaluate_expression(&exprs[..idx].to_vec(), first_is_negative);
                let r = evaluate_expression(&exprs[idx + 1..].to_vec(), true);
                return merge_maps(&l, &r);
            }
            false => match exprs.iter().any(|&x| x == "*") {
                true => {
                    let idx = exprs.iter().position(|&x| x == "*").unwrap();
                    let l = evaluate_expression(&exprs[..idx].to_vec(), first_is_negative);
                    let r = evaluate_expression(&exprs[idx + 1..].to_vec(), first_is_negative);
                    return multiply_maps(&l, &r);
                }
                false => {
                    if exprs.len() > 1 {
                        panic!("No ops, expected sub-expr to be a unit: {:?}", exprs[1]);
                    } else if exprs[0].starts_with('-') {
                        return evaluate_expression(&vec![&exprs[0][1..]], !first_is_negative);
                    } else if exprs[0].parse::<i128>().is_ok() {
                        let value = {
                            if first_is_negative {
                                F::from(exprs[0].parse::<i128>().unwrap().abs() as u128).neg()
                            } else {
                                F::from(exprs[0].parse::<i128>().unwrap() as u128)
                            }
                        };
                        let mut result = HashMap::new();
                        result.insert(None, value);
                        return result;
                    } else if is_valid_variable_name(exprs[0]) {
                        let mut result = HashMap::new();
                        let value = if first_is_negative {
                            F::one().neg()
                        } else {
                            F::one()
                        };
                        result.insert(Some(exprs[0].to_string()), value);
                        return result;
                    } else {
                        println!("exprs:{:?}", exprs);
                        panic!("ok wtf is {}", exprs[0]);
                    }
                }
            },
        },
    }
}

/// Converts an equation to a mapping of term to coefficient, and verifies that
/// the operations in the equation are valid.
///
/// Also outputs a triple containing the L and R input variables and the output
/// variable
///
/// Think of the list of (variable triples, coeffs) pairs as this language's
/// version of "assembly"
///
/// Example valid equations, and output:
/// a === 9                      ([None, None, 'a'], {'': 9})
/// b <== a * c                  (['a', 'c', 'b'], {'a*c': 1})
/// d <== a * c - 45 * a + 987   (['a', 'c', 'd'], {'a*c': 1, 'a': -45, '': 987})
///
/// Example invalid equations:
/// 7 === 7                      // Can't assign to non-variable
/// a <== b * * c                // Two times signs in a row
/// e <== a + b * c * d          // Multiplicative degree > 2
///
pub fn eq_to_assembly<F: PrimeField>(eq: &str) -> AssembleEquation<F> {
    let tokens: Vec<&str> = eq.trim().split(" ").collect();
    if tokens[1] == "<==" || tokens[1] == "===" {
        // First token is the output variable
        let mut out = tokens[0];
        // Convert the expression to coefficient map form
        let mut coeffs = evaluate_expression(&tokens[2..].to_vec(), false);
        // Handle the "-x === a * b" case
        if out.chars().nth(0).unwrap() == '-' {
            out = &out[1..];
            coeffs.insert(Some("$output_coeff".to_string()), F::one().neg());
        }
        // Check out variable name validity
        if !is_valid_variable_name(out) {
            panic!("Invalid out variable name: {}", out);
        }
        // Gather list of variables used in the expression
        let mut variables: Vec<&str> = Vec::new();
        for &t in tokens[2..].iter() {
            let var = &t.trim_start_matches("-");
            if is_valid_variable_name(var) && !variables.contains(var) {
                variables.push(var);
            }
        }
        // Construct the list of allowed coefficients
        let mut allowed_coeffs: Vec<String> = variables.iter().map(|&s| s.to_string()).collect();
        allowed_coeffs.extend(vec!["".to_string(), "$output_coeff".to_string()]);

        if variables.is_empty() {
            todo!();
        } else if variables.len() == 1 {
            variables.push(variables[0]);
            let product_key =
                get_product_key(Some(variables[0].to_owned()), Some(variables[1].to_owned()))
                    .unwrap();
            allowed_coeffs.push(product_key);
        } else if variables.len() == 2 {
            let product_key =
                get_product_key(Some(variables[0].to_owned()), Some(variables[1].to_owned()))
                    .unwrap();
            allowed_coeffs.push(product_key);
        } else {
            panic!("Max 2 variables, found {}", variables.len());
        }

        // Check that only allowed coefficients are in the coefficient map
        for key_option in coeffs.keys() {
            // Use as_ref to convert Option<String> to Option<&String> so that you can safely access the String reference inside it.
            let key_ref = key_option.as_ref().unwrap();

            // Check if allowed_coeffs contains this reference
            if !allowed_coeffs.contains(key_ref) {
                panic!("Disallowed multiplication");
            }
        }

        // Return output
        let variables_len = variables.len();
        let mut wires: Vec<Option<&str>> = variables
            .into_iter()
            .map(Some)
            .chain(vec![None; 2 - variables_len])
            .collect();
        wires.push(Some(out));

        return AssembleEquation {
            wires: GateWire {
                L: Some(wires[0].unwrap().to_string()),
                R: Some(wires[1].unwrap().to_string()),
                O: Some(wires[2].unwrap().to_string()),
            },
            coeffs,
        };
    } else if tokens[1] == "public" {
        let mut coeffs = HashMap::new();
        coeffs.insert(Some(tokens[0].to_string()), F::one().neg());
        coeffs.insert(Some("$output_coeff".to_string()), F::zero());
        coeffs.insert(Some("$public".to_string()), F::one());
        return AssembleEquation {
            wires: GateWire {
                L: Some(tokens[0].to_string()),
                R: None,
                O: None,
            },
            coeffs,
        };
    } else {
        panic!("Unsupported op: {}", tokens[1]);
    }
}

// =========================
// Here comes the impls
// =========================
impl GateWire {
    pub fn as_list(&self) -> Vec<Option<String>> {
        vec![self.L.clone(), self.R.clone(), self.O.clone()]
    }
}
