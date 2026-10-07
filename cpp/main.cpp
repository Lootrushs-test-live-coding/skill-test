#include "ledger.hpp"

#include <cstdint>
#include <iostream>

namespace {

int failures = 0;

void expect(bool condition, const char* label) {
    if (!condition) {
        std::cerr << "FAIL " << label << "\n";
        ++failures;
    }
}

Ledger example() {
    Ledger ledger;
    ledger.put_escrow(1, Escrow{10, 20, 30, 500, Status::Funded});
    ledger.put_escrow(2, Escrow{10, 20, 30, 100, Status::Refunded});
    ledger.set_balance(20, 25);
    return ledger;
}

}  // namespace

int main() {
    {
        Ledger ledger = example();
        expect(ledger.release(10, 1) == Error::Ok, "buyer release");
        expect(ledger.balance(20) == 525, "buyer balance");
        auto escrow = ledger.escrow(1);
        expect(escrow.has_value() && escrow->status == Status::Released, "buyer status");
        expect(escrow.has_value() && escrow->amount == 0, "buyer amount");
        expect(ledger.escrow(2)->status == Status::Refunded, "other escrow untouched");
    }
    {
        Ledger ledger = example();
        expect(ledger.release(30, 1) == Error::Ok, "arbiter release");
        expect(ledger.balance(20) == 525, "arbiter balance");
    }
    {
        Ledger ledger = example();
        expect(ledger.release(20, 1) == Error::NotAuthorized, "seller rejected");
        expect(ledger.balance(20) == 25, "seller balance unchanged");
        auto escrow = ledger.escrow(1);
        expect(escrow->status == Status::Funded && escrow->amount == 500, "seller state unchanged");
    }
    {
        Ledger ledger = example();
        expect(ledger.release(10, 1) == Error::Ok, "first release");
        expect(ledger.release(10, 1) == Error::BadStatus, "second release");
        expect(ledger.balance(20) == 525, "balance credited once");
    }
    {
        Ledger ledger = example();
        expect(ledger.release(10, 99) == Error::NotFound, "missing");
        expect(ledger.balance(20) == 25, "missing balance");
    }
    {
        Ledger ledger = example();
        expect(ledger.release(10, 2) == Error::BadStatus, "refunded");
        expect(ledger.balance(20) == 25, "refunded balance");
        expect(ledger.escrow(2)->amount == 100, "refunded amount");
    }
    {
        Ledger ledger = example();
        ledger.set_balance(20, UINT64_MAX - 10);
        expect(ledger.release(10, 1) == Error::Overflow, "overflow");
        expect(ledger.balance(20) == UINT64_MAX - 10, "overflow balance");
        auto escrow = ledger.escrow(1);
        expect(escrow->status == Status::Funded && escrow->amount == 500, "overflow state");
    }

    if (failures != 0) {
        std::cerr << failures << " failed\n";
        return 1;
    }
    std::cout << "ok\n";
    return 0;
}
