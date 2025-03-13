// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Params} from "../params.sol";
import {OnchainOracleSchema} from "../database/schemas/onchain-oracle.schema.sol";
import {Context} from "../commons/context.sol";
import {OnchainOracleEvents} from "../events/onchain-oracle.event.sol";
import {AbstractOnchainOracle} from "../../controllers/abstract-onchain-oracle.controller.sol";
import {OnchainOracleErrors as Errors} from "../errors/onchain-oracle.error.sol";

library OnchianOracleProvider {
    function initialize(address off_chain_oracle) internal {
        OnchainOracleSchema.initialize(off_chain_oracle);
    }
    function request_for_randomness(Params.RandomnessRequest memory params) internal returns(bytes32 request_id) {
        request_id = gen_request_id(params.app_hash, params.requestor);
        if (OnchainOracleSchema.is_request_exists(request_id)) {
            revert Errors.REQUEST_ALREADY_EXISTS();
        }

        OnchainOracleSchema.create_request(request_id, params.requestor);
        emit OnchainOracleEvents.RequestRandoness(request_id, params.requestor);
    }

    function fullfill_randomness_future(Params.RandomnessFutureFullfillment memory params) internal {
        address requestor = Context.sender();
        if (!OnchainOracleSchema.is_ofchain_oracle(requestor)) {
            revert Errors.INVALID_OFFCHAIN_ORACLE();
        }

        OnchainOracleSchema.Request memory request = OnchainOracleSchema.get_request(params.request_id);
        request.random_word = params.random_word;
        request.random_word_signer = params.signer;
        request.signature = params.signature;
        request.status = true;

        OnchainOracleSchema.set_request(request);
        
        // this call should be made to a specified gas limit
        AbstractOnchainOracle(request.requestor).fullfill_randomness_future(request);

        emit OnchainOracleEvents.FullfillRandomness(params.random_word, params.request_id, params.requestor, params.signature, params.signer);
    }   

    function get_request(bytes32 request_id) internal view returns (OnchainOracleSchema.Request memory) {
        return OnchainOracleSchema.get_request(request_id);
    }

    function verify_randomness() internal pure returns (bool) {
        // todo!()
        return true;
    }

    function gen_request_id(bytes32 app_hash, address requestor) internal view returns (bytes32) {
        return keccak256(abi.encodePacked(block.timestamp, block.prevrandao, app_hash, requestor));
    }
    
}