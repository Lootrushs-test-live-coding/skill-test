package ledger

import (
	"math"
	"testing"
)

func example() *Ledger {
	ledger := New()
	ledger.PutEscrow(1, Escrow{Buyer: 10, Seller: 20, Arbiter: 30, Amount: 500, Status: Funded})
	ledger.PutEscrow(2, Escrow{Buyer: 10, Seller: 20, Arbiter: 30, Amount: 100, Status: Refunded})
	ledger.SetBalance(20, 25)
	return ledger
}

func TestBuyerReleases(t *testing.T) {
	ledger := example()
	if err := ledger.Release(10, 1); err != nil {
		t.Fatal(err)
	}
	if ledger.Balance(20) != 525 {
		t.Fatalf("balance %d", ledger.Balance(20))
	}
	escrow, _ := ledger.Escrow(1)
	if escrow.Status != Released || escrow.Amount != 0 {
		t.Fatalf("escrow %+v", escrow)
	}
	other, _ := ledger.Escrow(2)
	if other.Status != Refunded {
		t.Fatal("other escrow changed")
	}
}

func TestArbiterReleases(t *testing.T) {
	ledger := example()
	if err := ledger.Release(30, 1); err != nil {
		t.Fatal(err)
	}
	if ledger.Balance(20) != 525 {
		t.Fatalf("balance %d", ledger.Balance(20))
	}
}

func TestSellerIsRejected(t *testing.T) {
	ledger := example()
	if err := ledger.Release(20, 1); err != ErrNotAuthorized {
		t.Fatalf("err %v", err)
	}
	if ledger.Balance(20) != 25 {
		t.Fatal("balance changed")
	}
	escrow, _ := ledger.Escrow(1)
	if escrow.Status != Funded || escrow.Amount != 500 {
		t.Fatalf("escrow %+v", escrow)
	}
}

func TestSecondReleaseIsRejected(t *testing.T) {
	ledger := example()
	if err := ledger.Release(10, 1); err != nil {
		t.Fatal(err)
	}
	if err := ledger.Release(10, 1); err != ErrBadStatus {
		t.Fatalf("err %v", err)
	}
	if ledger.Balance(20) != 525 {
		t.Fatal("credited twice")
	}
}

func TestMissingEscrow(t *testing.T) {
	ledger := example()
	if err := ledger.Release(10, 99); err != ErrNotFound {
		t.Fatalf("err %v", err)
	}
	if ledger.Balance(20) != 25 {
		t.Fatal("balance changed")
	}
}

func TestRefundedIsRejected(t *testing.T) {
	ledger := example()
	if err := ledger.Release(10, 2); err != ErrBadStatus {
		t.Fatalf("err %v", err)
	}
	if ledger.Balance(20) != 25 {
		t.Fatal("balance changed")
	}
	escrow, _ := ledger.Escrow(2)
	if escrow.Amount != 100 {
		t.Fatal("amount changed")
	}
}

func TestOverflowLeavesState(t *testing.T) {
	ledger := example()
	ledger.SetBalance(20, math.MaxInt64-10)
	if err := ledger.Release(10, 1); err != ErrOverflow {
		t.Fatalf("err %v", err)
	}
	if ledger.Balance(20) != math.MaxInt64-10 {
		t.Fatal("balance changed")
	}
	escrow, _ := ledger.Escrow(1)
	if escrow.Status != Funded || escrow.Amount != 500 {
		t.Fatalf("escrow %+v", escrow)
	}
}
