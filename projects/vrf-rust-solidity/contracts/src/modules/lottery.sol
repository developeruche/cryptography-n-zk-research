// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {LotteryController} from "../controllers/lottery.controller.sol";
import {AbstractOnchainOracle} from "./abstract-onchain-oracle.sol";
import {OnchainOracleSchema} from "../providers/database/schemas/onchain-oracle.schema.sol";
import {LotteryProvider as provider} from "../providers/modules/lottery.provider.sol";
import {OnchainOracleController} from "../controllers/onchain-oracle.controller.sol";
import {Params} from "../providers/params.sol";




contract Lottery is AbstractOnchainOracle {
    OnchainOracleSchema.Request public request;


    constructor(address _onchain_oracle, uint256 entry_prize) AbstractOnchainOracle(_onchain_oracle) {
        provider.initialize(entry_prize);
    }


    function participate() external payable {
        provider.participate();
    }

    function end() external {
        provider.end();
        OnchainOracleController(onchain_oracle).request_for_randomness(
            Params.RandomnessRequest({
                requestor: address(this),
                app_hash: keccak256("lottery_demo")
            })
        );
    }

    function pay_out_winner() external {
        provider.pay_out_winner();
    }

    function fullfill_randomness_future_internal(OnchainOracleSchema.Request memory _request) internal override {
        request = _request;

        // compute winner
        provider.compute_winner(_request.random_word);
    } 
}
