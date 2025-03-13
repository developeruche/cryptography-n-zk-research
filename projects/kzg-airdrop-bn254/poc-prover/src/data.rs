use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ff::BigInt;
use csv::Reader;
use ethers::{
    abi::{encode, Token},
    types::{Address, U256},
    utils::keccak256,
};
use std::str::FromStr;

use crate::error::{Error, Result};

pub fn srs_g1() -> Vec<G1Affine> {
    vec![]
}

pub fn srs_g2() -> Vec<G2Affine> {
    vec![]
}

pub fn read_user_data(path: &str) -> Result<Vec<Fr>> {
    let mut data = Reader::from_path(path)?;
    data.records()
        .map(|record| {
            let record = record?;
            let addr = record
                .get(0)
                .ok_or(Error::Internal("No value".to_string()))?
                .trim();

            let amount = record
                .get(1)
                .ok_or(Error::Internal("No value".to_string()))?
                .trim();

            let encoded_data = encode(&[
                Token::Address(Address::from_str(addr).unwrap()),
                Token::Uint(U256::from_dec_str(amount).unwrap()),
            ]);

            let mut hash = keccak256(encoded_data);
            hash.reverse();

            let mut u64_array = [0u64; 4];
            for (i, val) in hash.chunks(8).enumerate() {
                let arr: [u8; 8] = val.try_into().expect("error &[u8] to [u8;8]");
                let chunk = u64::from_le_bytes(arr);
                u64_array[i] = chunk;
            }
            let big_int = BigInt(u64_array);
            Ok(Fr::from(big_int))
        })
        .collect()
}
