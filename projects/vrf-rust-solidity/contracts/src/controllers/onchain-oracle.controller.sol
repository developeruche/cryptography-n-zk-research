// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Params} from "../providers/params.sol";
import {OnchainOracleSchema} from "../providers/database/schemas/onchain-oracle.schema.sol";

interface OnchainOracleController {
    function fullfill_randomness_future(Params.RandomnessFutureFullfillment memory params) external;
    function request_for_randomness(Params.RandomnessRequest memory params) external returns (bytes32) ;
    function get_request(bytes32 request_id) external view returns (OnchainOracleSchema.Request memory);
    function verify_randomness() external pure returns (bool);
}