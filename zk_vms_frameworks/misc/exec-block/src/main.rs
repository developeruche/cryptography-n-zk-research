//! This bin is tasked with taking a block and chain-state witness, performing validation of the block statelessly
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_stateless::{StatelessInput, fork_spec::ForkSpec, validation::stateless_validation};
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;



fn main() {
    println!("Starting stateless block validation");
    let input = obtain_input();
    let chain_spec: Arc<ChainSpec> = Arc::new(obtain_fork_spec().into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());
    
    match stateless_validation(input.block, input.witness, chain_spec, evm_config) {
        Ok(data) => println!("Block validation completed successfully: {:?}", data),
        Err(e) => panic!("Block validation failed: {:?}", e),
    }
}



fn obtain_input() -> StatelessInput {
    // Try different possible paths for the block and witness data JSON file
    let path = "block_and_witness.json";

    // Try to open the file from one of the possible paths
    let file;

    let file_path = Path::new(path);
    match File::open(file_path) {
        Ok(f) => {
            file = Some(f);
        }
        Err(e) => {
            panic!("Could not open {} - {}",  path, e);
        }
    }

    // Read the file content
    let mut json_content = String::new();
    if let Err(e) = file.unwrap().read_to_string(&mut json_content) {
        panic!("Failed to read file content: {}, {}", e, path);
    }

    match serde_json::from_str::<StatelessInput>(&json_content) {
        Ok(input) => {
            input
        }
        Err(e) => {
            panic!("Failed to parse {} as StatelessInput: {}", path, e)
        }
    }
}

fn obtain_fork_spec() -> ForkSpec {
    ForkSpec::Prague
}
