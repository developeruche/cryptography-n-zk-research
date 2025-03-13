// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import {Lottery} from "../src/modules/lottery.sol";

contract InteractScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address off_chain_signer = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;


        vm.startBroadcast(deployerPrivateKey);

        Lottery game = Lottery(0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512);
        game.participate{value: 1 ether}();
        game.end();

        
        vm.stopBroadcast();
    }
}


// 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512