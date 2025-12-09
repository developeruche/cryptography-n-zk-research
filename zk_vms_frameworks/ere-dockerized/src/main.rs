use std::fs;
use std::path::PathBuf;
use ere_dockerized::{CompilerKind, DockerizedCompiler, DockerizedzkVM, zkVMKind};
use ere_zkvm_interface::{
    compiler::Compiler,
    zkvm::{Input, ProofKind, ProverResourceType, zkVM},
};


pub fn main() {
    // run_compilation_sp1();
    // run_execution_sp1();
    // run_prove_sp1();
    run_verify_sp1();
}

fn run_compilation_sp1() {
    let program_dir = PathBuf::from("/Users/gregg/Documents/projects/RESEARCH/cryptography-n-zk-research/zk_vms_frameworks/ere-dockerized/guest-programs/sp1-fib");
    let compiler =
            DockerizedCompiler::new(zkVMKind::SP1, CompilerKind::RustCustomized, &program_dir).unwrap();
    let program = compiler.compile(&program_dir).unwrap();
    let program_vec = serde_json::to_vec(&program).unwrap();
    fs::write(program_dir.join("sp1-program.elf"), program_vec).expect("Failed to write ELF file");
}

fn run_execution_sp1() {
    let program_dir = PathBuf::from("/Users/gregg/Documents/projects/RESEARCH/cryptography-n-zk-research/zk_vms_frameworks/ere-dockerized/guest-programs/sp1-fib");
    let compiler =
            DockerizedCompiler::new(zkVMKind::SP1, CompilerKind::RustCustomized, &program_dir).unwrap();
    let program = compiler.compile(&program_dir).unwrap();
    
    let zkvm = DockerizedzkVM::new(zkVMKind::SP1, program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output = Input::new().with_prefixed_stdin(55u64.to_le_bytes().to_vec());
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
}


fn run_prove_sp1() {
    let program_dir = PathBuf::from("/Users/gregg/Documents/projects/RESEARCH/cryptography-n-zk-research/zk_vms_frameworks/ere-dockerized/guest-programs/sp1-fib");
    let compiler =
            DockerizedCompiler::new(zkVMKind::SP1, CompilerKind::RustCustomized, &program_dir).unwrap();
    let program = compiler.compile(&program_dir).unwrap();
    
    let zkvm = DockerizedzkVM::new(zkVMKind::SP1, program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output: Vec<u8> = 55u64.to_le_bytes().to_vec();
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
    
    let (public_values, proof, report) = zkvm.prove(&input, ProofKind::default()).unwrap();
    // assert_eq!(public_values, expected_output);
    println!("Proving time: {:?}", report.proving_time);
}


fn run_verify_sp1() {
    let program_dir = PathBuf::from("/Users/gregg/Documents/projects/RESEARCH/cryptography-n-zk-research/zk_vms_frameworks/ere-dockerized/guest-programs/sp1-fib");
    let compiler =
            DockerizedCompiler::new(zkVMKind::SP1, CompilerKind::RustCustomized, &program_dir).unwrap();
    let program = compiler.compile(&program_dir).unwrap();
    
    let zkvm = DockerizedzkVM::new(zkVMKind::SP1, program, ProverResourceType::Cpu).unwrap();
    let input = Input::new().with_prefixed_stdin(10u64.to_le_bytes().to_vec());
    let expected_output: Vec<u8> = 55u64.to_le_bytes().to_vec();
    let (public_values, report) = zkvm.execute(&input).unwrap();
    println!("Public value: {:?}", public_values);
    println!("Expected output: {:?}", expected_output);
    println!("Execution cycles: {}", report.total_num_cycles);
    
    let (public_values, proof, report) = zkvm.prove(&input, ProofKind::default()).unwrap();
    // assert_eq!(public_values, expected_output);
    println!("Proving time: {:?}", report.proving_time);
    
    let public_values = zkvm.verify(&proof).unwrap();
    // assert_eq!(public_values, expected_output);
    println!("Proof verified successfully!");
}