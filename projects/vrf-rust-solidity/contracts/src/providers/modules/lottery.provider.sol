// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;


import { LotterySchema as Schema } from "../database/schemas/lottery.schema.sol";
import {Context} from "../commons/context.sol";
import {LotteryErrors as Errors} from "../errors/lottery.error.sol";


library LotteryProvider {
    function initialize(uint256 entry_prize) internal {
        address requestor = Context.sender();
        Schema.initialize(entry_prize, requestor);
    }


    function participate() internal {
        address requestor = Context.sender();
        uint256 amount = msg.value;

        if (amount != Schema.get_entry_prize()) {
            revert Errors.AMOUNT_NOT_ENOUGH();
        }

        if (Schema.get_winner() != address(0)) {
            revert Errors.LOTTERY_ALREADY_ENDED();
        }

        if (Schema.participant_has_joined(requestor)) {
            revert Errors.PARTICIPANT_HAS_ALREADY_JOINED();
        }


        Schema.push_participant(requestor);
    }



    function end() internal {
        address requestor = Context.sender();
        if (requestor != Schema.get_owner()) {
            revert Errors.INVALID_OWNER();
        }

        Schema.set_is_end(true);
    }
    

    function pay_out_winner() internal {
        address winner = Schema.get_winner();
        if (winner == address(0)) {
            revert Errors.NO_WINNER_YET();
        }
        uint256 amount = address(this).balance;
        payable(winner).transfer(amount);
    }

    function compute_winner(uint256 rand) internal returns (address) {
        address[] memory participants = Schema.get_participants();
        uint256 length = participants.length;
        uint256 random_index = rand % length;
        address winner = participants[random_index];
        Schema.set_winner(winner);
        return winner;
    }

}