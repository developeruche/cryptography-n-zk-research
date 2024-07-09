use std::iter::repeat;

use crate::functions::{convert_to_u32, split_u64_to_u32};

pub struct Block {
    pub w: [u32; 64]
}

pub struct PreProcessor {
    pub blob: Vec<u8>
}


pub trait PreProcessorInterface {
    /// This takes in the data to be hashed and arrange it as a preprocessed vector of 512bit
    fn compute_blocks(&mut self) -> Vec<Block>;
}

pub trait BlockInterface {
    /// This function uses the block info to create a message shedule
    fn message_shedule(&self) -> [u32; 64];
}


impl PreProcessorInterface for PreProcessor {
    fn compute_blocks(&mut self) -> Vec<Block> {
        let mut blocks = Vec::new();
        
        if self.blob.len() <= 112 {
            let len_initial = self.blob.len() as u64;
            let len_initial_2_u32 = split_u64_to_u32(len_initial);
            let padding_lenght = 112 - len_initial;
            self.blob.extend(repeat(0u8).take(padding_lenght as usize));
            let mut blob_u32 = convert_to_u32(self.blob.clone());
            blob_u32.extend(len_initial_2_u32);
            
            let block: Block = Block {
                w: blob_u32.try_into().unwrap()
            };
            
            blocks.push(block);
        } else {
            
        }
        
        
        blocks
    }
}