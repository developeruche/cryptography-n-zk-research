// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface HasherController {
    function MiMC5Sponge(uint256[2] memory _ins, uint256 _k) external view returns (uint256 h);
}
