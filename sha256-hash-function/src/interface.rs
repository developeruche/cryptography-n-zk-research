pub struct Block {
    pub w: [u32; 64]
}

pub struct PreProcessor {
    pub blob: Vec<u8>
}


pub trait PreProcessorInterface {
    /// This takes in the data to be hashed and arrange it as a preprocessed vector of 512bit
    fn compute_blocks(&self) -> Vec<Block>;
}

pub trait BlockInterface {
    /// This function uses the block info to create a message shedule
    fn message_shedule(&self) -> [u32; 64];
}


impl PreProcessorInterface for PreProcessor {
    fn compute_blocks(&self) -> Vec<Block> {
        let blocks = Vec::new();
        
        if self.blob.len() <= 112 {
            let mut w_i: [u32; 64] = [0; 64];
            
            for (i, x) in self.blob.iter().enumerate() {
                w_i[i] = u32::from_be_bytes(bytes)
            }
        } else {
            
        }
        
        
        blocks
    }
}