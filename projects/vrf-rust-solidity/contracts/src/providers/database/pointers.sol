// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

library Pointers {
    bytes32 constant LOTTERY_STORAGE_POSITION = keccak256("lottery.storage");
    bytes32 constant ONCHAIN_ORACLE_STORAGE_POSITION = keccak256("onchain.oracle.storage");
    bytes32 constant CONTEXT_STORAGE_POSITION = keccak256("context.storage");
}
