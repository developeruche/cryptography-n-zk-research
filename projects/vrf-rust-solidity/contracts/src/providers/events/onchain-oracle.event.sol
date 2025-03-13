// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

library OnchainOracleEvents {
    event RequestRandoness(bytes32 request_id, address requestor);
    event FullfillRandomness(uint256 random_word, bytes32 request_id, address requestor, bytes signature, address signer);
}
