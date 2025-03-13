// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/modules/onchain-oracle.sol";
import {Lottery} from "../src/modules/lottery.sol";

contract MainTest is Test {
    OnchainOracle public oracle;
    Lottery public game;



    address owner = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    address player_one = 0x70997970C51812dc3A010C7d01b50e0d17dc79C8;
    address player_two = 0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC;
    address off_chain_oracale = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;


    // function setUp() public {
    //     vm.startPrank(owner);

    //     vm.deal(owner, 1000 ether);
    //     vm.deal(player_one, 1000 ether);
    //     vm.deal(player_two, 1000 ether);
    //     vm.deal(off_chain_oracale, 1000 ether);


    //     oracle = new OnchainOracle(off_chain_oracale);
    //     game = new Lottery(off_chain_oracale, 1 ether);
    // }

    // function test_lottery() public {
    //     game.participate{value: 1 ether}();
    //     game.end();
    // }
}
