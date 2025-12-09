#![no_main]

extern crate alloc;

use std::sync::Arc;

use ere_platform_sp1::sp1_zkvm;
use reth_chainspec::ChainSpec;
use reth_evm_ethereum::EthEvmConfig;
use reth_primitives_traits::Block;
use reth_stateless::{
    stateless_validation_with_trie, Genesis, StatelessInput, UncompressedPublicKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sparsestate::SparseState;

sp1_zkvm::entrypoint!(main);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RethStatelessValidatorInput {
    /// The stateless input for the stateless validation function.
    pub stateless_input: StatelessInput,
    /// The recovered signers for the transactions in the block.
    pub public_keys: Vec<UncompressedPublicKey>,
}
pub type RethStatelessValidatorOutput = ([u8; 32], [u8; 32], bool);

pub fn main() {
    let input = sp1_zkvm::io::read::<RethStatelessValidatorInput>();
    let genesis = Genesis {
        config: input.stateless_input.chain_config.clone(),
        ..Default::default()
    };
    let chain_spec: Arc<ChainSpec> = Arc::new(genesis.into());
    let evm_config = EthEvmConfig::new(chain_spec.clone());

    let header = input.stateless_input.block.header().clone();
    let parent_hash = input.stateless_input.block.parent_hash;

    let res = stateless_validation_with_trie::<SparseState, _, _>(
        input.stateless_input.block,
        input.public_keys,
        input.stateless_input.witness,
        chain_spec,
        evm_config,
    )
    .map(|(block_hash, _)| block_hash);

    let output: RethStatelessValidatorOutput = match res {
        Ok(block_hash) => (block_hash.0, parent_hash.0, true),
        Err(err) => {
            let err_string = &format!("Block validation failed: {err}\n");
            println!("{}", err_string);
            (header.hash_slow().0, parent_hash.0, false)
        }
    };

    let output_serialized = serde_json::to_vec(&output).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&output_serialized);
    let output_hash: [u8; 32] = hasher.finalize().into();

    sp1_zkvm::io::commit_slice(&output_hash);
}
