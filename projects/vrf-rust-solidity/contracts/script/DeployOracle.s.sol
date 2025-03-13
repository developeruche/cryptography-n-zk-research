// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import {OnchainOracle} from "../src/modules/onchain-oracle.sol";

contract OracleScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address off_chain_signer = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;


        vm.startBroadcast(deployerPrivateKey);

        OnchainOracle oracle = new OnchainOracle(off_chain_signer);

        
        vm.stopBroadcast();
    }
}
