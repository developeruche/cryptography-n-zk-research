use crate::{
    constants::{H, W},
    utils::{bits2bytes, keccak_f, multirate_padding},
};

#[derive(Debug, Clone)]
pub struct KeccakState {
    pub bitrate: usize,
    pub b: usize,
    pub bitrate_bytes: usize,
    pub lanew: usize,
    pub s: Vec<Vec<u64>>,
}

#[derive(Debug, Clone)]
pub struct KeccakSponge {
    pub state: KeccakState,
    pub buffer: Vec<u8>,
}

pub struct KeccakHash {
    pub sponge: KeccakSponge,
    pub digest_size: usize,
    pub block_size: usize,
}

impl KeccakState {
    fn zero() -> Vec<Vec<u64>> {
        vec![vec![0; W]; H]
    }

    fn format(st: &Vec<Vec<u64>>) -> String {
        let mut rows = Vec::new();
        for y in 0..H {
            let row: Vec<String> = (0..W).map(|x| format!("{:016x}", st[x][y])).collect();
            rows.push(row.join(" "));
        }
        rows.join("\n")
    }

    fn lane2bytes(s: u64, w: usize) -> Vec<u8> {
        let mut o = Vec::new();
        for b in (0..w).step_by(8) {
            o.push(((s >> b) & 0xFF) as u8);
        }
        o
    }

    fn bytes2lane(bb: &[u8]) -> u64 {
        bb.iter().rev().fold(0u64, |acc, &b| (acc << 8) | b as u64)
    }

    pub fn new(bitrate: usize, b: usize) -> Self {
        assert!(bitrate % 8 == 0);
        let bitrate_bytes = bits2bytes(bitrate);

        assert!(b % 25 == 0);
        let lanew = b / 25;

        KeccakState {
            bitrate,
            b,
            bitrate_bytes,
            lanew,
            s: Self::zero(),
        }
    }

    pub fn absorb(&mut self, bb: &[u8]) {
        assert_eq!(bb.len(), self.bitrate_bytes);

        let mut extended_bb = bb.to_vec();
        extended_bb.extend(vec![0; bits2bytes(self.b - self.bitrate_bytes)]);

        let mut i = 0;
        for y in 0..H {
            for x in 0..W {
                self.s[x][y] ^= Self::bytes2lane(&extended_bb[i..i + 8]);
                i += 8;
            }
        }
    }

    pub fn squeeze(&self) -> Vec<u8> {
        self.get_bytes()[..self.bitrate_bytes].to_vec()
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let mut out = vec![0u8; bits2bytes(self.b)];
        let mut i = 0;
        for y in 0..H {
            for x in 0..W {
                let v = Self::lane2bytes(self.s[x][y], self.lanew);
                out[i..i + 8].copy_from_slice(&v);
                i += 8;
            }
        }
        out
    }

    pub fn set_bytes(&mut self, bb: &[u8]) {
        let mut i = 0;
        for y in 0..H {
            for x in 0..W {
                self.s[x][y] = Self::bytes2lane(&bb[i..i + 8]);
                i += 8;
            }
        }
    }
}

impl std::fmt::Display for KeccakState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::format(&self.s))
    }
}

impl KeccakSponge {
    pub fn new(bitrate: usize, width: usize) -> Self {
        KeccakSponge {
            state: KeccakState::new(bitrate, width),
            buffer: Vec::new(),
        }
    }

    pub fn absorb_block(&mut self, bb: &[u8]) {
        assert_eq!(bb.len(), self.state.bitrate_bytes);
        self.state.absorb(bb);
        keccak_f(&mut self.state);
    }

    pub fn absorb(&mut self, s: &[u8]) {
        self.buffer.extend_from_slice(s);

        while self.buffer.len() >= self.state.bitrate_bytes {
            let temp_buf = self.buffer.clone();
            let (block, rest) = temp_buf.split_at(self.state.bitrate_bytes);
            self.absorb_block(block);
            self.buffer = rest.to_vec();
        }
    }

    pub fn absorb_final(&mut self) {
        let padding = multirate_padding(self.buffer.len(), self.state.bitrate_bytes);
        let mut padded = self.buffer.clone();
        padded.extend_from_slice(&padding);
        self.absorb_block(&padded);
        self.buffer.clear();
    }

    pub fn squeeze_once(&mut self) -> Vec<u8> {
        let rc = self.state.squeeze();
        keccak_f(&mut self.state);
        rc
    }

    pub fn squeeze(&mut self, l: usize) -> Vec<u8> {
        let mut z = self.squeeze_once();
        while z.len() < l {
            z.extend_from_slice(&self.squeeze_once());
        }
        z.truncate(l);
        z
    }
}

impl KeccakHash {
    pub fn new(bitrate_bits: usize, capacity_bits: usize, output_bits: usize) -> Self {
        // Validate parameters
        assert!(
            [25, 50, 100, 200, 400, 800, 1600].contains(&(bitrate_bits + capacity_bits)),
            "Invalid total bits"
        );
        assert!(output_bits % 8 == 0, "Output bits must be byte-aligned");

        KeccakHash {
            sponge: KeccakSponge::new(bitrate_bits, bitrate_bits + capacity_bits),
            digest_size: bits2bytes(output_bits),
            block_size: bits2bytes(bitrate_bits),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.sponge.absorb(data);
    }

    pub fn digest(&self) -> Vec<u8> {
        let mut finalised = self.sponge.clone();
        finalised.absorb_final();
        finalised.squeeze(self.digest_size)
    }

    pub fn hexdigest(&self) -> String {
        self.digest()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }

    pub fn clone(&self) -> Self {
        KeccakHash {
            sponge: self.sponge.clone(),
            digest_size: self.digest_size,
            block_size: self.block_size,
        }
    }

    // Factory function for preset configurations
    pub fn preset(
        bitrate_bits: usize,
        capacity_bits: usize,
        output_bits: usize,
    ) -> impl Fn(Option<&[u8]>) -> KeccakHash {
        move |initial_input: Option<&[u8]>| {
            let mut hasher = KeccakHash::new(bitrate_bits, capacity_bits, output_bits);
            if let Some(input) = initial_input {
                hasher.update(input);
            }
            hasher
        }
    }
}
