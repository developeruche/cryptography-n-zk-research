use keccak::KeccakHash;
pub mod constants;
pub mod keccak;
pub mod utils;


pub fn keccak224(initial_input: Option<&[u8]>) -> KeccakHash {
    KeccakHash::preset(1152, 448, 224)(initial_input)
}

pub fn keccak256(initial_input: Option<&[u8]>) -> KeccakHash {
    KeccakHash::preset(1088, 512, 256)(initial_input)
}

pub fn keccak384(initial_input: Option<&[u8]>) -> KeccakHash {
    KeccakHash::preset(832, 768, 384)(initial_input)
}

pub fn keccak512(initial_input: Option<&[u8]>) -> KeccakHash {
    KeccakHash::preset(576, 1024, 512)(initial_input)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn test_keccak224() {
    //     let input = b"Hello, world!";
    //     let hash = keccak224(Some(input));
    //     assert_eq!(hash.len(), 28);
    // }
    
    #[test]
    fn test_keccak256() {
        let mut hasher = keccak256(None);
        hasher.update(b"Hello, world!");
        let digest = hasher.hexdigest();
        println!("{}", digest);
    }
    
    // #[test]
    // fn test_keccak384() {
    //     let input = b"Hello, world!";
    //     let hash = keccak384(Some(input));
    //     assert_eq!(hash.len(), 48);
    // }
    
    // #[test]
    // fn test_keccak512() {
    //     let input = b"Hello, world!";
    //     let hash = keccak512(Some(input));
    //     assert_eq!(hash.len(), 64);
    // }
}