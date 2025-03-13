// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;


import {OnchainOracleSchema} from "../providers/database/schemas/onchain-oracle.schema.sol";



interface AbstractOnchainOracle {
    function fullfill_randomness_future(OnchainOracleSchema.Request memory request) external;
}