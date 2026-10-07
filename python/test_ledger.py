import unittest

from ledger import BadStatus, Escrow, Ledger, NotAuthorized, NotFound


def example() -> Ledger:
    ledger = Ledger()
    ledger.put_escrow(1, Escrow(10, 20, 30, 500, "funded"))
    ledger.put_escrow(2, Escrow(10, 20, 30, 100, "refunded"))
    ledger.set_balance(20, 25)
    return ledger


class ReleaseTest(unittest.TestCase):
    def test_buyer_releases(self) -> None:
        ledger = example()
        ledger.release(10, 1)
        self.assertEqual(ledger.balance(20), 525)
        escrow = ledger.escrow(1)
        assert escrow is not None
        self.assertEqual(escrow.status, "released")
        self.assertEqual(escrow.amount, 0)
        other = ledger.escrow(2)
        assert other is not None
        self.assertEqual(other.status, "refunded")

    def test_arbiter_releases(self) -> None:
        ledger = example()
        ledger.release(30, 1)
        self.assertEqual(ledger.balance(20), 525)
        escrow = ledger.escrow(1)
        assert escrow is not None
        self.assertEqual(escrow.status, "released")

    def test_seller_is_rejected(self) -> None:
        ledger = example()
        with self.assertRaises(NotAuthorized):
            ledger.release(20, 1)
        self.assertEqual(ledger.balance(20), 25)
        escrow = ledger.escrow(1)
        assert escrow is not None
        self.assertEqual(escrow.status, "funded")
        self.assertEqual(escrow.amount, 500)

    def test_second_release_is_rejected(self) -> None:
        ledger = example()
        ledger.release(10, 1)
        with self.assertRaises(BadStatus):
            ledger.release(10, 1)
        self.assertEqual(ledger.balance(20), 525)

    def test_missing_escrow(self) -> None:
        ledger = example()
        with self.assertRaises(NotFound):
            ledger.release(10, 99)
        self.assertEqual(ledger.balance(20), 25)

    def test_refunded_is_rejected(self) -> None:
        ledger = example()
        with self.assertRaises(BadStatus):
            ledger.release(10, 2)
        self.assertEqual(ledger.balance(20), 25)
        escrow = ledger.escrow(2)
        assert escrow is not None
        self.assertEqual(escrow.amount, 100)


if __name__ == "__main__":
    unittest.main()
