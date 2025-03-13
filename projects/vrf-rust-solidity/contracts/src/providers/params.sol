// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;




library Params {
    struct RandomnessRequest {
        bytes32 app_hash;
        address requestor;
    }

    struct RandomnessFutureFullfillment {
        uint256 random_word;
        bytes32 request_id;
        address requestor;
        bytes signature;
        address signer;
    }
}