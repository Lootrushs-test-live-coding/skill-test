package ledger

import "errors"

type Status int

const (
	Funded Status = iota
	Released
	Refunded
)

type Escrow struct {
	Buyer   int64
	Seller  int64
	Arbiter int64
	Amount  int64
	Status  Status
}

var (
	ErrNotFound      = errors.New("not found")
	ErrNotAuthorized = errors.New("not authorized")
	ErrBadStatus     = errors.New("bad status")
	ErrOverflow      = errors.New("overflow")
)

type Ledger struct {
	balances map[int64]int64
	escrows  map[int64]Escrow
}

func New() *Ledger {
	return &Ledger{
		balances: map[int64]int64{},
		escrows:  map[int64]Escrow{},
	}
}

func (l *Ledger) PutEscrow(id int64, escrow Escrow) {
	l.escrows[id] = escrow
}

func (l *Ledger) SetBalance(account, amount int64) {
	l.balances[account] = amount
}

func (l *Ledger) Balance(account int64) int64 {
	return l.balances[account]
}

func (l *Ledger) Escrow(id int64) (Escrow, bool) {
	escrow, ok := l.escrows[id]
	return escrow, ok
}

func (l *Ledger) Release(caller, escrowID int64) error {
	// Implement this function.
	return ErrNotFound
}
