#![no_main]

use ere_platform_sp1::{sp1_zkvm, Platform, SP1Platform};

sp1_zkvm::entrypoint!(main);

type P = SP1Platform;

pub fn main() {
    // Read serialized input and deserialize it.
    let input = P::read_whole_input();
    let n = u64::from_le_bytes((&*input).try_into().unwrap());

    // Compute nth fib.
    let fib_n = fib(n);

    // Write serialized output.
    let mut output = input.to_vec();
    output.extend_from_slice(&fib_n.to_le_bytes());
    P::write_whole_output(&output);
}

fn fib(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}