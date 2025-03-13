// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

library LotteryErrors {
    error AMOUNT_NOT_ENOUGH();
    error LOTTERY_ALREADY_ENDED();
    error PARTICIPANT_HAS_ALREADY_JOINED();
    error INVALID_OWNER();
    error NO_WINNER_YET();
}
