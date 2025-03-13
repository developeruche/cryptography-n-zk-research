// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Pointers} from "../database/pointers.sol";
import {ContextSchema} from "../database/schemas/context.schema.sol";

library Context {


    function writeableStorage() internal pure returns (ContextSchema.ContextData storage ds) {
        bytes32 position = Pointers.CONTEXT_STORAGE_POSITION;

        assembly {
            ds.slot := position
        }
    }

    // I would not be excposing this functionality (I don't need it) (it is just here for flexibility)
    function setTrustedForwarder(address forwarder) internal {
        writeableStorage().isTrustedForwarder[forwarder] = true;
    }

    function isTrustedForwarder(address forwarder) internal view returns (bool) {
        return writeableStorage().isTrustedForwarder[forwarder];
    }

    function sender() internal view returns (address senderAddress) {
        if (isTrustedForwarder(msg.sender)) {
            assembly {
                senderAddress := shr(96, calldataload(sub(calldatasize(), 20)))
            }
        } else {
            return msg.sender;
        }
    }

    function data() internal view returns (bytes calldata) {
        if (isTrustedForwarder(msg.sender)) {
            return msg.data[:msg.data.length - 20];
        } else {
            return msg.data;
        }
    }

    function timestamp() internal view returns (uint256) {
        return block.timestamp;
    }

    function blocks() internal view returns (uint256) {
        return block.number;
    }
}