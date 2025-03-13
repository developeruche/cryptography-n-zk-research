// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Params} from "../providers/params.sol";
import {OnchianOracleProvider as provider, OnchainOracleSchema} from "../providers/modules/onchain-oracle.provider.sol";
import {OnchainOracleController as Controller} from "../controllers/onchain-oracle.controller.sol";

contract OnchainOracle is Controller {
    constructor(address off_chain_signer) {
        provider.initialize(off_chain_signer);
    }
    
    function request_for_randomness(Params.RandomnessRequest memory params) external returns (bytes32){
        return provider.request_for_randomness(params);
    }

    function fullfill_randomness_future(Params.RandomnessFutureFullfillment memory params) external {
        provider.fullfill_randomness_future(params);
    }


    function get_request(bytes32 request_id) external view returns (OnchainOracleSchema.Request memory){
        return provider.get_request(request_id); 
    }

    function verify_randomness() external pure returns (bool) {
        return provider.verify_randomness();
    }
}
