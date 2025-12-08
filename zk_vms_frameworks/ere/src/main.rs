use std::fs;
use std::path::PathBuf;
use ere_sp1::{compiler::RustRv32imaCustomized, zkvm::EreSP1};
use ere_zkvm_interface::{Compiler, Input, ProofKind, ProverResourceType, zkVM};


pub fn main() {
    run_compilation_sp1();
    run_execution_sp1();
    run_prove_sp1();
    run_verify_sp1();
}

fn run_compilation_sp1() {
    let program_dir = PathBuf::from("guest-programs/sp1-fib");
    let program = RustRv32imaCustomized.compile(&program_dir)
        .expect("Failed to compile SP1 program");
    fs::write(program_dir.join("sp1-program.elf"), program.elf()).expect("Failed to write ELF file");
}

fn run_execution_sp1() {
    let program_dir = PathBuf::from("guest-programs/sp1-fib");
    let compiler = RustRv32imaCustomized;
    let program = compiler.compile(&program_dir).unwrap();
    
    let zkvm = EreSP1::new(program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output = Input::new().with_prefixed_stdin(55u64.to_le_bytes().to_vec());
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
}


fn run_prove_sp1() {
    let program_dir = PathBuf::from("guest-programs/sp1-fib");
    let compiler = RustRv32imaCustomized;
    let program = compiler.compile(&program_dir).unwrap();

    let zkvm = EreSP1::new(program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output: Vec<u8> = 55u64.to_le_bytes().to_vec();
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
    
    let (public_values, proof, report) = zkvm.prove(&input, ProofKind::default()).unwrap();
    assert_eq!(public_values, expected_output);
    println!("Proving time: {:?}", report.proving_time);
}


fn run_verify_sp1() {
    let program_dir = PathBuf::from("guest-programs/sp1-fib");
    let compiler = RustRv32imaCustomized;
    let program = compiler.compile(&program_dir).unwrap();

    let zkvm = EreSP1::new(program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output: Vec<u8> = 55u64.to_le_bytes().to_vec();
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
    
    let (public_values, proof, report) = zkvm.prove(&input, ProofKind::default()).unwrap();
    assert_eq!(public_values, expected_output);
    println!("Proving time: {:?}", report.proving_time);
    
    let public_values = zkvm.verify(&proof).unwrap();
    assert_eq!(public_values, expected_output);
    println!("Proof verified successfully!");
}