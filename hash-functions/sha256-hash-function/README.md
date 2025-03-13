In the realm of cybersecurity, ensuring data integrity and authenticity is paramount. Cryptographic hash functions play a vital role in achieving this objective. One of the most widely employed and trusted algorithms in this domain is SHA-256 (Secure Hash Algorithm 256-bit). This introduction will delve into the concept of SHA-256, outlining its functionalities, key characteristics, and the prevalent applications that leverage its capabilities.



In the realm of cybersecurity, ensuring data integrity and authenticity is paramount. Cryptographic hash functions play a vital role in achieving this objective. One of the most widely employed and trusted algorithms in this domain is SHA-256 (Secure Hash Algorithm 256-bit). This introduction will delve into the concept of SHA-256, outlining its functionalities, key characteristics, and the prevalent applications that leverage its capabilities.



- Necessity for data integrity and authenticity: Briefly explain why data security is crucial and how it can be compromised.
    
- Hash functions explained: Introduce the concept of hash functions, their role in cryptography, and how they differ from encryption.

- SHA-256: A Secure Hashing Standard: Mention that SHA-256 belongs to the SHA-2 family, developed as a successor to the aging SHA-1 algorithm.


### Operations Used In SHA256
It is important to note that SHA256 operations are done of 32 bits words:: *This is still questionable*
1.  Exclusive OR (XOR): This operation returns `true` if the two inputs are different and `false` if otherwise. This is used in the SHA256 algorithm to give a balanced representation of the different `operands` used during this hash computation. In Rust, you can perform the XOR (exclusive OR) operation using the `^` operator. XOR is a bitwise operation that takes two-bit patterns of equal length and performs the logical exclusive OR operation on each pair of corresponding bits. The result is 1 if the bits are different and 0 if they are the same.
```rust 
//This is how to XOR in the RUST programming language
fn main() {
    let a: u8 = 0b1100_1010;
    let b: u8 = 0b1010_1100;

    let result = a ^ b;

    println!("{:08b}", a);       // Output: 11001010
    println!("{:08b}", b);       // Output: 10101100
    println!("{:08b}", result);  // Output: 01100110
}
```
2. Right Shift Operator (SHR_i): This shifts the bits of a 32-bit word by `n` number of bits to the right. Example; SHR_1(1011) == 1011 >> 1 == 0101. In Rust, the right shift operation can be performed using the `>>` operator. This operator shifts the bits of a number to the right by a specified number of positions. The bits that are shifted out of the rightmost position are discarded, and zeros are inserted from the left.
```rust 
fn main() {
    let value: u8 = 0b1100_1010;
    let shifted = value >> 3;

    println!("{:08b}", value);    // Output: 11001010
    println!("{:08b}", shifted);  // Output: 00011001
}
```
3. Rotation Right  (ROTR_i): The rotation shift right operation, often abbreviated as "ROR" or "rotate right," is a bitwise operation that shifts the bits of a binary number to the right by a specified number of positions. Bits that are shifted out of the rightmost position are reintroduced at the leftmost position, creating a circular or rotating effect.
```sh
# Consider an 8-bit binary number `11010010`. If we perform a right rotation by 3 positions, the result would be:

Original:   11010010
Step 1:     01101001 (shift right by 1)
Step 2:     10110100 (shift right by 2)
Step 3:     01011010 (shift right by 3)
Result:     01011010
```
This is how it is done in RUST
```rust
fn rotate_right(value: u8, shift: u32) -> u8 {
    // Ensure the shift amount is within the bounds of the bit width
    let shift = shift % 8;
    // Perform the rotation
    (value >> shift) | (value << (8 - shift))
}
```
4. Addition Modulo 2^32: This is done by adding the operands and taking the modulus of 2 ^ 32.

### Functions used in SHA256 
There are 6 logical functions used by the SHA256 hash function They are as follows;
1. sigma_0: The sigma_0 function comprises of 4 operations, ROTR_7, ROTR_18, SHR_3 and XOR. The equation looking more like this: ROTR_7(x) ^ ROTR_18(x) ^ SHR_3(x);
2. sigma_1: The sigma_1 function very similar to the sigma_0 function but varies in the number of rotation. This equation is given in this manner; ROTR_17(x) ^ ROTR_19(x) ^ SHR_10(x)
3. prime_sigma_0: ROTR_2(x) ^ ROTR_13(x) ^ ROTR_22(x) 
4. prime_sigma_1: ROTR_6(x) ^ ROTR_11(x) ^ ROTR_25(x)
5. choice Ch(𝑥,𝑦,𝑧)=(𝑥∧𝑦)⊕(¬𝑥∧𝑧) : Ch stands for choose (source: poncho) or choice, as the 𝑥 input chooses if the output is from 𝑦 or from 𝑧. More precisely, for each bit index, that result bit is according to the bit from 𝑦 (or respectively 𝑧) at this index, depending on if the bit from 𝑥 at this index is 1 (or respectively 0).
6. majority Maj(𝑥,𝑦,𝑧)=(𝑥∧𝑦)⊕(𝑥∧𝑧)⊕(𝑦∧𝑧): Maj stands for majority: for each bit index, that result bit is according to the majority of the 3 inputs bits for 𝑥 𝑦 and 𝑧 at this index.
7. compute_message_shedule_extension: 

_where ∧ is bitwise AND, ⊕ is bitwise exclusive-OR, and ¬ is bitwise negation. The functions are defined for bit vectors (of 32 bits in case fo SHA-256)._

### Constant used in SHA256
the 64 binary words K_i given by the 32 first bits of the fractional parts of the cube roots of the first 64 prime numbers;

    0x428a2f98, 
    0x71374491, 
    0xb5c0fbcf, 
    0xe9b5dba5, 
    0x3956c25b, 
    0x59f111f1, 
    0x923f82a4, 
    0xab1c5ed5,
    0xd807aa98, 
    0x12835b01, 
    0x243185be, 
    0x550c7dc3, 
    0x72be5d74, 
    0x80deb1fe, 
    0x9bdc06a7, 
    0xc19bf174,
    0xe49b69c1, 
    0xefbe4786, 
    0x0fc19dc6, 
    0x240ca1cc, 
    0x2de92c6f, 
    0x4a7484aa, 
    0x5cb0a9dc, 
    0x76f988da,
    0x983e5152, 
    0xa831c66d, 
    0xb00327c8, 
    0xbf597fc7, 
    0xc6e00bf3, 
    0xd5a79147, 
    0x06ca6351, 
    0x14292967,
    0x27b70a85, 
    0x2e1b2138, 
    0x4d2c6dfc, 
    0x53380d13, 
    0x650a7354, 
    0x766a0abb, 
    0x81c2c92e, 
    0x92722c85,
    0xa2bfe8a1, 
    0xa81a664b, 
    0xc24b8b70, 
    0xc76c51a3, 
    0xd192e819, 
    0xd6990624, 
    0xf40e3585, 
    0x106aa070,
    0x19a4c116, 
    0x1e376c08, 
    0x2748774c, 
    0x34b0bcb5, 
    0x391c0cb3, 
    0x4ed8aa4a, 
    0x5b9cca4f, 
    0x682e6ff3,
    0x748f82ee, 
    0x78a5636f, 
    0x84c87814, 
    0x8cc70208, 
    0x90befffa, 
    0xa4506ceb, 
    0xbef9a3f7, 
    0xc67178f2


### Preprocessing
This entails all the stages of processes needed to be done before the hash computation commences. This are 4 stages within this stage.

1. Convert this message to binary
2. Padding: This involves padding the message until it reaches the nearest multiple of `512`
	first, a bit 1 is appended,
	• next, k bits 0 are appended, with k being the smallest positive integer such that l + 1 + k ≡ 448
	mod 512, where l is the length in bits of the initial message,
	• finally, the length l < 2
	64 of the initial message is represented with exactly 64 bits, and these bits
	are added at the end of the message.
	The message shall always be padded, even if the initial length is already a multiple of 512.

```rust 
pub struct PreProcessor {
	blob: Vec<u8>
}

pub struct Block {
	w: Vec<u32> // [u32; 64]
}

trait PreProcessorInterface {
	// This is the vector of 512bits
	fn compute_blocks(&self) -> Vec<Block>
}

trait BlockInterface {
	// This is what is called the message schedule
	fn compute_message_shedule() -> [u32; 64];
}
```
3. Expand the Message schedule.
4. Set initial hash values.
```rust 
pub const H: [u32; 8] = [
    0x6a09e667, 
    0xbb67ae85, 
    0x3c6ef372, 
    0xa54ff53a, 
    0x510e527f, 
    0x9b05688c, 
    0x1f83d9ab, 
    0x5be0cd19
];
```


### Hash Computation 

