// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../src/modules/onchain-oracle.sol";

contract OracleTest is Test {
    OnchainOracle public oracle;



    address owner = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    address off_chain_oracale = 0x90F79bf6EB2c4f870365E785982E1f101E93b906;


    // function setUp() public {
    //     vm.startPrank(owner);

    //     vm.deal(owner, 1000 ether);
    //     vm.deal(off_chain_oracale, 1000 ether);


    //     oracle = new OnchainOracle(off_chain_oracale);
    // }

    // function test_request_for_randomness() public {
    //     oracle.request_for_randomness(
    //         Params.RandomnessRequest({
    //             requestor: address(this),
    //             app_hash: keccak256("lottery_demo")
    //         })
    //     );
    // }
}
