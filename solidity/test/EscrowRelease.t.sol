// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import "../src/EscrowRelease.sol";

contract StoringSeller {
    uint256 public seen;

    receive() external payable {
        seen = uint256(EscrowRelease(msg.sender).status());
    }
}

contract RejectingSeller {
    receive() external payable {
        revert("no");
    }
}

contract EscrowReleaseTest is Test {
    address constant BUYER = address(0xB0B);
    address constant SELLER = address(0x5E11);
    address constant ARBITER = address(0xA4B);

    function _funded(address seller) internal returns (EscrowRelease) {
        return new EscrowRelease{value: 500}(BUYER, seller, ARBITER);
    }

    function test_buyerRelease() public {
        EscrowRelease escrow = _funded(SELLER);
        uint256 before = SELLER.balance;

        vm.prank(BUYER);
        escrow.release();

        assertEq(uint8(escrow.status()), uint8(EscrowRelease.Status.Released));
        assertEq(escrow.amount(), 0);
        assertEq(address(escrow).balance, 0);
        assertEq(SELLER.balance, before + 500);
    }

    function test_arbiterRelease() public {
        EscrowRelease escrow = _funded(SELLER);

        vm.prank(ARBITER);
        escrow.release();

        assertEq(uint8(escrow.status()), uint8(EscrowRelease.Status.Released));
        assertEq(SELLER.balance, 500);
    }

    function test_sellerCannotRelease() public {
        EscrowRelease escrow = _funded(SELLER);

        vm.prank(SELLER);
        vm.expectRevert(EscrowRelease.NotAuthorized.selector);
        escrow.release();

        assertEq(uint8(escrow.status()), uint8(EscrowRelease.Status.Funded));
        assertEq(escrow.amount(), 500);
        assertEq(address(escrow).balance, 500);
    }

    function test_secondReleaseReverts() public {
        EscrowRelease escrow = _funded(SELLER);

        vm.prank(BUYER);
        escrow.release();

        vm.prank(BUYER);
        vm.expectRevert(EscrowRelease.BadStatus.selector);
        escrow.release();

        assertEq(SELLER.balance, 500);
    }

    function test_rejectingSellerKeepsFunds() public {
        RejectingSeller seller = new RejectingSeller();
        EscrowRelease escrow = _funded(address(seller));

        vm.prank(BUYER);
        vm.expectRevert(EscrowRelease.TransferFailed.selector);
        escrow.release();

        assertEq(uint8(escrow.status()), uint8(EscrowRelease.Status.Funded));
        assertEq(escrow.amount(), 500);
        assertEq(address(escrow).balance, 500);
    }

    function test_statusIsReleasedBeforeEthIsSent() public {
        StoringSeller seller = new StoringSeller();
        EscrowRelease escrow = _funded(address(seller));

        vm.prank(BUYER);
        escrow.release();

        assertEq(seller.seen(), uint256(EscrowRelease.Status.Released));
        assertEq(address(seller).balance, 500);
    }
}
