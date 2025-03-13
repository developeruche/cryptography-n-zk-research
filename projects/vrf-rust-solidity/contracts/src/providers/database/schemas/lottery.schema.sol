// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Pointers} from "../pointers.sol";
import {LotteryErrors} from "../../errors/lottery.error.sol";


library LotterySchema {
    struct Storage {
        address[] participants;
        uint256 startedAt;
        address winner;
        uint256 entry_prize;
        mapping(address => bool) participants_map;
        address owner;
        bool is_end;
    }


    function lottery_storage() internal pure returns (Storage storage s) {
        bytes32 position = Pointers.LOTTERY_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }


    function initialize(uint256 entry_prize, address owner) internal {
        Storage storage s = lottery_storage();
        s.startedAt = block.timestamp;
        s.entry_prize = entry_prize;
        s.owner = owner;    
    }

    function get_participants() internal view returns (address[] memory) {
        Storage storage s = lottery_storage();
        return s.participants;
    }

    function push_participant(address participant) internal {
        Storage storage s = lottery_storage();
        s.participants.push(participant);
    }

    function get_winner() internal view returns (address) {
        Storage storage s = lottery_storage();
        return s.winner;
    }

    function set_winner(address winner) internal {
        Storage storage s = lottery_storage();
        s.winner = winner;
    }

    function get_entry_prize() internal view returns (uint256) {
        Storage storage s = lottery_storage();
        return s.entry_prize;
    }

    function set_participant(address participant) internal {
        Storage storage s = lottery_storage();
        s.participants_map[participant] = true;
    }

    function participant_has_joined(address participant) internal view returns (bool) {
        Storage storage s = lottery_storage();
        return s.participants_map[participant];
    }

    function set_is_end(bool is_end) internal {
        Storage storage s = lottery_storage();
        s.is_end = is_end;
    }

    function get_owner() internal view returns (address) {
        Storage storage s = lottery_storage();
        return s.owner;
    }
}
