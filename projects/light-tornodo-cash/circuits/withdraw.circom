pragma circom  2.0.0;




// this is the circuit that wold be used for verified with zero knownlegde the depoisit of the users
// the circuit is based merkle tree

// include "./node_modules/circomlib/circuits/mimcsponge.circom";
include "./utils/mimc5sponge.circom";
include "./hasher.circom";



// we are using 20 here because that is the height of the merkle tree
template withdraw() {
    signal input root; // this is the root of the merkle tree after you made your deposist
    signal input nullifierHash; // this is the nullifer hash (one of the params used to generate the commitment used during the deposit)
    signal input recipient; // this address to reach the deposit 
    signal input secret[256]; // this is the secret used to generate the commitment
    signal input nullifer[256]; // this is the nullifier used to generate the commitment
    signal input pairHashes[20]; // this is the sister hashes for the merkle tree
    signal input hashNavigation[20]; // this is used navigation through the merkle tree


    // first constraint is to check that the nullifier is correct (comparing the hash computed with the hash provided)
    component hasher = Hasher();
    hasher.secret <== secret;
    hasher.nullifier <== nullifier;


    // next stage would be diving into the merkle tree
    component leafHashers[20];
    signal currentHash[20 + 1];
    currentHash[0] <== hasher.hashSecret;

    signal left[20];
    signal right[20];

    for(var i = 0; i < 20; i++) {
        var d = hashNavigation[i];
        leafHashers[i] = MiMC5Sponge(2);


        // performing the left switching here
        left[i] <== (1 - 0) * currentHash[i];
        leafHashers[i].ins[0] <== left[i] + d * pairHashes[i];

        right[i] <== d * currentHash[i];
        leafHashers[i].ins[1] <== right[i] + (1 - d) * pairHashes[i];

        leafHashers[i].k <== hasher.hashSecret;
        currentHash[i + 1] <== leafHashers[i].o;
    }


    // at this the point the tree has been built; not we can perform our check 
    root === currentHash[20];


    // now adding an additonal contant to cover for the recipient
    signal recipientSquare;
    recipientSquare <== recipient * recipient;




}


component main {public [root, nullifierHash, recipient]} = Withdraw();