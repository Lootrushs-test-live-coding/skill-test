import java.util.HashMap;
import java.util.Map;

enum Status {
    FUNDED,
    RELEASED,
    REFUNDED
}

final class Escrow {
    long buyer;
    long seller;
    long arbiter;
    long amount;
    Status status;

    Escrow(long buyer, long seller, long arbiter, long amount, Status status) {
        this.buyer = buyer;
        this.seller = seller;
        this.arbiter = arbiter;
        this.amount = amount;
        this.status = status;
    }
}

class NotFoundException extends RuntimeException {}

class NotAuthorizedException extends RuntimeException {}

class BadStatusException extends RuntimeException {}

public final class Ledger {
    private final Map<Long, Escrow> escrows = new HashMap<>();
    private final Map<Long, Long> balances = new HashMap<>();

    public void putEscrow(long id, Escrow escrow) {
        escrows.put(id, escrow);
    }

    public void setBalance(long account, long amount) {
        balances.put(account, amount);
    }

    public long balance(long account) {
        return balances.getOrDefault(account, 0L);
    }

    public Escrow escrow(long id) {
        return escrows.get(id);
    }

    public void release(long caller, long escrowId) {
        // Implement this function.
        throw new UnsupportedOperationException("implement release");
    }
}
