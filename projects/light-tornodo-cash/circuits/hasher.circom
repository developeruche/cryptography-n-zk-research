pragma circom  2.0.0;



// this role of this circuit would be to handle the hashing of the sercet and the nuilfer using the perdeson circuit from the circomlib
// the circuit would be a simple circuit that takes in the secret and the nullifier and outputs the hash of the secret and the nullifier



include "./node_modules/circomlib/circuits/pedersen.circom";


template Hasher() {
    signal input secret[256]; // the secret is a 256 bit number (in practice uint256 from solidity)
    signal input nullifier[256]; // the nullifier is a 256 bit number

    signal output hashSecret;
    signal output hashNullifier;


    component commitmentHasher = Pedersen(512); // this would be hashing the concatiantion of the sercet and the nullifier
    component nullifierHasher = Pedersen(256); // this would be hashing (only) the nullifier



    // the hash of the secret and the nullifier
    for (var i = 0; i < 256; i++) {
        commitmentHasher.in[i] <== nullifier[i]; // the first 256 bits of the input to the commitmentHasher is the nullifier
        commitmentHasher.in[i + 256] <== secret[i]; // the last 256 bits of the input to the commitmentHasher is the secret
        nullifierHasher.in[i] <== nullifier[i]; // the input to the nullifierHasher is the nullifier
    }


    // obtaining the commitments 
    hashSecret <== commitmentHasher.out[0];
    hashNullifier <== nullifierHasher.out[0];
}