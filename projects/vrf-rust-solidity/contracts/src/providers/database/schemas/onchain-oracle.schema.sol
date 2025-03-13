// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import {Pointers} from "../pointers.sol";
import {OnchainOracleErrors} from "../../errors/onchain-oracle.error.sol";

library OnchainOracleSchema {
    struct Request {
        bytes32 request_id;
        address requestor;
        bool status;
        uint256 random_word;
        address random_word_signer;
        bytes signature;
    }

    struct Storage {
        mapping(bytes32 => Request) requests;
        address offchain_oracle;
    }

    function onchain_oracle_storage() internal pure returns (Storage storage s) {
        bytes32 position = Pointers.ONCHAIN_ORACLE_STORAGE_POSITION;
        assembly {
            s.slot := position
        }
    }

    function initialize(address off_chain_oracle) internal {
        Storage storage s = onchain_oracle_storage();
        s.offchain_oracle = off_chain_oracle;
    }


    function get_request(bytes32 request_id) internal view returns (Request memory) {
        Storage storage s = onchain_oracle_storage();
        return s.requests[request_id];
    }

    function set_request(Request memory _request) internal {
        {
            if (_request.request_id == bytes32(0)) {
                revert OnchainOracleErrors.INVALID_REQUEST_ID();
            }
        }
        Storage storage s = onchain_oracle_storage();

        s.requests[_request.request_id] = _request;
    }

    function create_request(bytes32 request_id, address requestor) internal {
        {
            if (request_id == bytes32(0)) {
                revert OnchainOracleErrors.INVALID_REQUEST_ID();
            }
        }
        Storage storage s = onchain_oracle_storage();

        if (s.requests[request_id].request_id != bytes32(0)) {
            revert OnchainOracleErrors.REQUEST_ALREADY_EXISTS();
        }

        Request memory request = Request({
            request_id: request_id,
            requestor: requestor,
            status: false,
            random_word: 0,
            random_word_signer: address(0),
            signature: bytes("")
        });

        s.requests[request_id] = request;
    }


    function set_request_status(bytes32 request_id, bool status) internal {
        Storage storage s = onchain_oracle_storage();
        s.requests[request_id].status = status;
    }

    function get_request_status(bytes32 request_id) internal view returns (bool) {
        Storage storage s = onchain_oracle_storage();
        return s.requests[request_id].status;
    }

    function set_offchain_oracle(address offchain_oracle) internal {
        Storage storage s = onchain_oracle_storage();
        s.offchain_oracle = offchain_oracle;
    }

    function get_offchain_oracle() internal view returns (address) {
        Storage storage s = onchain_oracle_storage();
        return s.offchain_oracle;
    }

    function set_random_word(bytes32 request_id, uint256 random_word, address random_word_signer) internal {
        Storage storage s = onchain_oracle_storage();
        s.requests[request_id].random_word = random_word;
        s.requests[request_id].random_word_signer = random_word_signer;
    }

    function is_ofchain_oracle(address addr) internal view returns (bool) {
        Storage storage s = onchain_oracle_storage();
        return s.offchain_oracle == addr;
    }

    function is_request_exists(bytes32 request_id) internal view returns (bool) {
        Storage storage s = onchain_oracle_storage();
        return s.requests[request_id].request_id != bytes32(0);
    }
}

