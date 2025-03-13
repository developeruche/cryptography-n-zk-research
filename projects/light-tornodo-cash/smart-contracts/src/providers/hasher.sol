// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

library HasherProvider {
    function MiMC5Feistel(uint256 _iL, uint256 _iR, uint256 _k, uint256 p, uint256[20] memory c)
        internal
        pure
        returns (uint256 oL, uint256 oR)
    {
        uint8 nRounds = 20;

        uint256 lastL = _iL;
        uint256 lastR = _iR;

        uint256 mask;
        uint256 mask2;
        uint256 mask4;
        uint256 temp;

        for (uint8 i = 0; i < nRounds; i++) {
            mask = addmod(lastR, _k, p);
            mask = addmod(mask, c[i], p);
            mask2 = mulmod(mask, mask, p);
            mask4 = mulmod(mask2, mask2, p);
            mask = mulmod(mask4, mask, p);

            temp = lastR;
            lastR = addmod(lastL, mask, p);
            lastL = temp;
        }

        return (lastL, lastR);
    }

    function MiMC5Sponge(uint256[2] memory _ins, uint256 _k, uint256 p, uint256[20] memory c)
        internal
        pure
        returns (uint256 h)
    {
        uint256 lastR = 0;
        uint256 lastC = 0;

        for (uint8 i = 0; i < _ins.length; i++) {
            lastR = addmod(lastR, _ins[i], p);
            (lastR, lastC) = MiMC5Feistel(lastR, lastC, _k, p, c);
        }

        h = lastR;
    }
}
