use ark_ff::PrimeField;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Column {
    LEFT,
    RIGHT,
    OUTPUT,
}

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        match value {
            1 => Column::LEFT,
            2 => Column::RIGHT,
            3 => Column::OUTPUT,
            _ => panic!("wrong column"),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Cell {
    pub column: Column,
    pub row: usize,
}

impl Cell {
    pub fn label<F: PrimeField>(&self, group_order: u64) -> F {
        roots_of_unity::<F>(group_order)[self.row]
            * match self.column {
                Column::LEFT => F::from(1u32),
                Column::RIGHT => F::from(2u32),
                Column::OUTPUT => F::from(3u32),
            }
    }
}

pub fn root_of_unity<F: PrimeField>(group_order: u64) -> F {
    F::get_root_of_unity(group_order).unwrap()
}

pub fn roots_of_unity<F: PrimeField>(group_order: u64) -> Vec<F> {
    let mut res = vec![F::from(1u32)];
    let generator: F = root_of_unity(group_order);
    for _ in 1..group_order {
        res.push(res[res.len() - 1] * generator);
    }
    res
}

pub fn find_next_power_of_two(n: usize, m: usize) -> usize {
    let mut power = 1;
    let target = n + m + 1;
    while power < target {
        power <<= 1;
    }
    power
}

pub fn get_product_key(key1: Option<String>, key2: Option<String>) -> Option<String> {
    match (key1, key2) {
        (Some(k1), Some(k2)) => {
            let members = {
                let mut members = Vec::new();
                members.extend(k1.split('*'));
                members.extend(k2.split('*'));
                members.sort();
                members
            };
            Some(
                members
                    .into_iter()
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>()
                    .join("*"),
            )
        }
        (Some(k1), None) => Some(k1),
        (None, Some(k2)) => Some(k2),
        (None, None) => None,
    }
}

pub fn is_valid_variable_name(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(char::is_alphanumeric)
        && !name.chars().next().unwrap().is_numeric()
}

pub fn merge_maps<F: PrimeField>(
    map1: &HashMap<Option<String>, F>,
    map2: &HashMap<Option<String>, F>,
) -> HashMap<Option<String>, F> {
    let mut merged = HashMap::new();
    for (key, val) in map1.iter().chain(map2.iter()) {
        *merged.entry(key.clone()).or_insert(F::zero()) += val;
    }
    merged
}

pub fn multiply_maps<F: PrimeField>(
    map1: &HashMap<Option<String>, F>,
    map2: &HashMap<Option<String>, F>,
) -> HashMap<Option<String>, F> {
    let mut result = HashMap::new();
    for (k1, v1) in map1.iter() {
        for (k2, v2) in map2.iter() {
            let product_key = get_product_key(k1.clone(), k2.clone());
            *result.entry(product_key).or_insert(F::zero()) += *v1 * *v2;
        }
    }
    result
}

pub fn extract_number_and_variable<F: PrimeField>(input: &str) -> Option<(F, Vec<String>)> {
    let re = Regex::new(r"^(\d+)?((\*[a-zA-Z]+)*|([a-zA-Z]+(\*[a-zA-Z]+)*))$").unwrap();
    if let Some(caps) = re.captures(input) {
        let number = caps
            .get(1)
            .and_then(|m| m.as_str().parse::<u64>().ok())
            .map_or(F::from(1u32), F::from); // If no number is given, the default is 1

        let variables = caps.get(2).map_or(vec![], |m| {
            m.as_str()
                .split('*')
                .filter(|s| !s.is_empty()) // Filter out empty strings
                .map(String::from)
                .collect()
        });

        return Some((number, variables));
    }

    None
}
