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

    #[test]
    fn test_keccak256() {
        let mut hasher = keccak256(None);
        hasher.update(b"hello hash");
        let digest = hasher.hexdigest();
        assert_eq!(
            "a06c4933962b145ac49be4d314c34aef46e63910355ea96160adcfb7a33c705d",
            digest
        )
    }
}
