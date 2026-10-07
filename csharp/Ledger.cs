public enum Status
{
    Funded,
    Released,
    Refunded,
}

public sealed class Escrow
{
    public long Buyer { get; set; }
    public long Seller { get; set; }
    public long Arbiter { get; set; }
    public long Amount { get; set; }
    public Status Status { get; set; }
}

public sealed class NotFoundException : Exception
{
}

public sealed class NotAuthorizedException : Exception
{
}

public sealed class BadStatusException : Exception
{
}

public sealed class Ledger
{
    private readonly Dictionary<long, Escrow> escrows = new();
    private readonly Dictionary<long, long> balances = new();

    public void PutEscrow(long id, Escrow escrow) => escrows[id] = escrow;

    public void SetBalance(long account, long amount) => balances[account] = amount;

    public long Balance(long account) => balances.TryGetValue(account, out var amount) ? amount : 0;

    public Escrow? Escrow(long id) => escrows.TryGetValue(id, out var escrow) ? escrow : null;

    public void Release(long caller, long escrowId)
    {
        // Implement this function.
        throw new NotImplementedException("implement release");
    }
}
