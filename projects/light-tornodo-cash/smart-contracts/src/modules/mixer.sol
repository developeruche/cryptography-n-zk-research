// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "./ReentrancyGuard.sol";
import "./hasher.sol";
import {Errors} from "../providers/errors.sol";
import "./verifier.sol";

contract Mixer is ReentrancyGuard {
    Hasher hasher;
    Groth16Verifier verifier;


    uint8 public treeLevels = 20; // this is the same merkle tree level used be tornado cash
    uint256 public denomimnation = 0.1 ether;
    uint256 public nextLeafIndex = 0;

    // declaring the mappings
    mapping(uint256 => bool) public roots; // this mapping holds a vaule of true for all the valid roots
    mapping(uint8 => uint256) public topLevelHash;
    mapping(uint256 => bool) public nullifierHashes; // this mapping holds a value of true for all the nullifier hashes (this would prevent double spending)
    mapping(uint256 => bool) public commitments; // this mapping holds a value of true for all the commitments (this would prevent replay attacks)

    uint256[20] levelDefault = [
        23183772226880328093887215408966704399401918833188238128725944610428185466379,
        24000819369602093814416139508614852491908395579435466932859056804037806454973,
        90767735163385213280029221395007952082767922246267858237072012090673396196740,
        36838446922933702266161394000006956756061899673576454513992013853093276527813,
        68942419351509126448570740374747181965696714458775214939345221885282113404505,
        50082386515045053504076326033442809551011315580267173564563197889162423619623,
        73182421758286469310850848737411980736456210038565066977682644585724928397862,
        60176431197461170637692882955627917456800648458772472331451918908568455016445,
        105740430515862457360623134126179561153993738774115400861400649215360807197726,
        76840483767501885884368002925517179365815019383466879774586151314479309584255,
        23183772226880328093887215408966704399401918833188238128725944610428185466379,
        24000819369602093814416139508614852491908395579435466932859056804037806454973,
        90767735163385213280029221395007952082767922246267858237072012090673396196740,
        36838446922933702266161394000006956756061899673576454513992013853093276527813,
        68942419351509126448570740374747181965696714458775214939345221885282113404505,
        50082386515045053504076326033442809551011315580267173564563197889162423619623,
        73182421758286469310850848737411980736456210038565066977682644585724928397862,
        60176431197461170637692882955627917456800648458772472331451918908568455016445,
        105740430515862457360623134126179561153993738774115400861400649215360807197726,
        76840483767501885884368002925517179365815019383466879774586151314479309584255
    ];

    // ==============================
    // EVENTS
    // ==============================
    event Deposit(uint256 indexed root, uint256[20] indexed pairHashes, uint8[20] pairNagivavtion);

    event Withdraw(address indexed to, uint256 nullifierHash);

    constructor(address _hasher, address _verifier) {
        hasher = Hasher(_hasher);
        verifier = Groth16Verifier(_verifier);
    }

    function deposit(uint256 _commitmentHash) external payable nonReentrant {
        if (msg.value != denomimnation) {
            revert Errors.IVALID_AMOUNT();
        }
        if (commitments[_commitmentHash]) {
            revert Errors.COMMITMENT_HAS_BEEN_USED();
        }
        if (nextLeafIndex >= 2 ** treeLevels) {
            revert Errors.MAXIMUM_TREE_DEPTH_REACHED();
        }

        uint256 newRoot;
        uint256[20] memory pairHashes;
        uint8[20] memory hashNavigation;

        uint256 currentIndex = nextLeafIndex;
        uint256 currentHash = _commitmentHash;

        uint256 left;
        uint256 right;

        uint256[2] memory ins;

        for (uint8 i = 0; i < treeLevels; i++) {
            if (currentIndex % 2 == 0) {
                left = currentHash;
                right = levelDefault[i];

                pairHashes[i] = levelDefault[i];
                hashNavigation[i] = 0;
            } else {
                left = levelDefault[i];
                right = currentHash;

                pairHashes[i] = levelDefault[i];
                hashNavigation[i] = 1;
            }

            ins[0] = left;
            ins[1] = right;

            uint256 h = hasher.MiMC5Sponge(ins, currentIndex);

            currentHash = h;

            currentIndex = currentIndex / 2;
        }

        newRoot = currentHash;
        roots[newRoot] = true;
        nextLeafIndex++;

        commitments[_commitmentHash] = true;
        emit Deposit(newRoot, pairHashes, hashNavigation);
    }


    function withdraw(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[2] calldata _pubSignals
    ) external payable nonReentrant {
        uint256 root = _pubSignals[0];
        uint256 nullifierHash = _pubSignals[1];


        if (!roots[root]) {
            revert Errors.INVALID_ROOT();
        }

        if (nullifierHashes[nullifierHash]) {
            revert Errors.NULLIFER_HAS_BEEN_USED();
        }


        // checks 

        uint256 addressToUint = uint256(uint160(msg.sender));
        uint256[3] memory __pubSignal = [_pubSignals[0], _pubSignals[0], addressToUint];

        // performing the verification
        (bool isValid) = verifier.verifyProof(_pA, _pB, _pC, __pubSignal);


        if (!isValid) {
            revert Errors.INVALID_PROOF();
        }


        nullifierHashes[nullifierHash] = true;

        // sending the ether to the recipient
        (bool ok, ) = payable(msg.sender).call{value: denomimnation}("");

        if (!ok) {
            revert Errors.PAYMENT_FAILED();
        }

        emit Withdraw(msg.sender, nullifierHash);
    }
}
