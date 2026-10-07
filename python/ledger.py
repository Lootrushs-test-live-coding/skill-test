from dataclasses import dataclass
from typing import Optional


class NotFound(Exception):
    pass


class NotAuthorized(Exception):
    pass


class BadStatus(Exception):
    pass


@dataclass
class Escrow:
    buyer: int
    seller: int
    arbiter: int
    amount: int
    status: str


class Ledger:
    def __init__(self) -> None:
        self.escrows: dict[int, Escrow] = {}
        self.balances: dict[int, int] = {}

    def put_escrow(self, escrow_id: int, escrow: Escrow) -> None:
        self.escrows[escrow_id] = escrow

    def set_balance(self, account: int, amount: int) -> None:
        self.balances[account] = amount

    def balance(self, account: int) -> int:
        return self.balances.get(account, 0)

    def escrow(self, escrow_id: int) -> Optional[Escrow]:
        return self.escrows.get(escrow_id)

    def release(self, caller: int, escrow_id: int) -> None:
        # Implement this function.
        raise NotImplementedError("implement release")
