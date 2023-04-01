// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

/**
 * @title Wordle
 * @dev Contract that stores commitment to a picked word in Wordle game.
 */
contract Wordle {

    address private owner;
    bytes32 public commitment;
    bytes32 public imageId; // image ID of the prover system

    // events for EVM logging
    event OwnerSet(address indexed oldOwner, address indexed newOwner);
    event CommitmentSet(bytes32 indexed oldCommitment, bytes32 indexed newCommitment);
    event ImageIdSet(bytes32 indexed oldImageId, bytes32 indexed newImageId);

    // modifier to check if caller is owner
    modifier isOwner() {
        // If the first argument of 'require' evaluates to 'false', execution terminates and all
        // changes to the state and to Ether balances are reverted.
        // This used to consume all gas in old EVM versions, but not anymore.
        require(msg.sender == owner, "Caller is not owner");
        _;
    }

    /**
     * @dev Set contract deployer as owner
     */
    constructor(bytes32 _commitment, bytes32 _imageId) {
        owner = msg.sender; // 'msg.sender' is sender of current call, contract deployer for a constructor
        commitment = _commitment;
        imageId = _imageId;
        emit OwnerSet(address(0), owner);
        emit CommitmentSet(bytes32(0), _commitment);
        emit ImageIdSet(bytes32(0), _imageId);
    }

    /**
     * @dev Return owner address
     * @return address of owner
     */
    function getOwner() external view returns (address) {
        return owner;
    }

    /**
     * @dev Change owner
     * @param newOwner address of new owner
     */
    function setOwner(address newOwner) public isOwner {
        owner = newOwner;
        emit OwnerSet(owner, newOwner);
    }

    /**
     * @dev Change imageId
     * @param newImageId - image ID of the prover system.
     */
    function setImageId(bytes32 newImageId) public isOwner {
        imageId = newImageId;
        emit ImageIdSet(imageId, newImageId);
    }

    /**
     * @dev Change commitment to picked word
     * @param newCommitment bytes32 of new commitment
     */
    function setCommitment(bytes32 newCommitment) public {
        commitment = newCommitment;
        emit CommitmentSet(commitment, newCommitment);
        // NOTE when changing commitment, the salt and hash might be revealed too
        // so that anyone can keep track of game history (which words were selected). 
        // emit CommitmentSet(oldCommitment, oldWord, oldSalt, newCommitment);
    }
} 