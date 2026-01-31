#![allow(non_local_definitions)]
#![allow(unexpected_cfgs)]
pub mod parts;
use ark_ff::{Fp64, MontBackend, MontConfig};

use crate::parts::part_one::part_one;

#[derive(MontConfig)]
#[modulus = "3221225473"]
#[generator = "5"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

fn main() {
    part_one::<Fr>();
}
