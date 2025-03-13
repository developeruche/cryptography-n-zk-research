// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;



library ContextSchema {
    struct ContextData {
        mapping(address => bool) isTrustedForwarder;
    }
}