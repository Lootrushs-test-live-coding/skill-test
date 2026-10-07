public final class Main {
    private static int failures = 0;

    public static void main(String[] args) {
        buyerReleases();
        arbiterReleases();
        sellerIsRejected();
        secondReleaseIsRejected();
        missingEscrow();
        refundedIsRejected();
        overflowLeavesState();
        if (failures != 0) {
            System.err.println(failures + " failed");
            System.exit(1);
        }
        System.out.println("ok");
    }

    private static Ledger example() {
        Ledger ledger = new Ledger();
        ledger.putEscrow(1, new Escrow(10, 20, 30, 500, Status.FUNDED));
        ledger.putEscrow(2, new Escrow(10, 20, 30, 100, Status.REFUNDED));
        ledger.setBalance(20, 25);
        return ledger;
    }

    private static void expect(boolean condition, String label) {
        if (!condition) {
            System.err.println("FAIL " + label);
            failures++;
        }
    }

    private static void buyerReleases() {
        Ledger ledger = example();
        ledger.release(10, 1);
        expect(ledger.balance(20) == 525, "buyer balance");
        expect(ledger.escrow(1).status == Status.RELEASED, "buyer status");
        expect(ledger.escrow(1).amount == 0, "buyer amount");
        expect(ledger.escrow(2).status == Status.REFUNDED, "other escrow");
    }

    private static void arbiterReleases() {
        Ledger ledger = example();
        ledger.release(30, 1);
        expect(ledger.balance(20) == 525, "arbiter balance");
        expect(ledger.escrow(1).status == Status.RELEASED, "arbiter status");
    }

    private static void sellerIsRejected() {
        Ledger ledger = example();
        try {
            ledger.release(20, 1);
            expect(false, "seller should throw");
        } catch (NotAuthorizedException ex) {
            expect(ledger.balance(20) == 25, "seller balance");
            expect(ledger.escrow(1).status == Status.FUNDED, "seller status");
            expect(ledger.escrow(1).amount == 500, "seller amount");
        }
    }

    private static void secondReleaseIsRejected() {
        Ledger ledger = example();
        ledger.release(10, 1);
        try {
            ledger.release(10, 1);
            expect(false, "second release should throw");
        } catch (BadStatusException ex) {
            expect(ledger.balance(20) == 525, "credited once");
        }
    }

    private static void missingEscrow() {
        Ledger ledger = example();
        try {
            ledger.release(10, 99);
            expect(false, "missing should throw");
        } catch (NotFoundException ex) {
            expect(ledger.balance(20) == 25, "missing balance");
        }
    }

    private static void refundedIsRejected() {
        Ledger ledger = example();
        try {
            ledger.release(10, 2);
            expect(false, "refunded should throw");
        } catch (BadStatusException ex) {
            expect(ledger.balance(20) == 25, "refunded balance");
            expect(ledger.escrow(2).amount == 100, "refunded amount");
        }
    }

    private static void overflowLeavesState() {
        Ledger ledger = example();
        ledger.setBalance(20, Long.MAX_VALUE - 10);
        try {
            ledger.release(10, 1);
            expect(false, "overflow should throw");
        } catch (ArithmeticException ex) {
            expect(ledger.balance(20) == Long.MAX_VALUE - 10, "overflow balance");
            expect(ledger.escrow(1).status == Status.FUNDED, "overflow status");
            expect(ledger.escrow(1).amount == 500, "overflow amount");
        }
    }
}
