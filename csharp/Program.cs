var failures = 0;

void Expect(bool condition, string label)
{
    if (!condition)
    {
        Console.Error.WriteLine("FAIL " + label);
        failures++;
    }
}

bool Throws<T>(Action action) where T : Exception
{
    try
    {
        action();
        return false;
    }
    catch (T)
    {
        return true;
    }
}

Ledger Example()
{
    var ledger = new Ledger();
    ledger.PutEscrow(1, new Escrow { Buyer = 10, Seller = 20, Arbiter = 30, Amount = 500, Status = Status.Funded });
    ledger.PutEscrow(2, new Escrow { Buyer = 10, Seller = 20, Arbiter = 30, Amount = 100, Status = Status.Refunded });
    ledger.SetBalance(20, 25);
    return ledger;
}

{
    var ledger = Example();
    ledger.Release(10, 1);
    Expect(ledger.Balance(20) == 525, "buyer balance");
    Expect(ledger.Escrow(1)!.Status == Status.Released, "buyer status");
    Expect(ledger.Escrow(1)!.Amount == 0, "buyer amount");
    Expect(ledger.Escrow(2)!.Status == Status.Refunded, "other escrow");
}
{
    var ledger = Example();
    ledger.Release(30, 1);
    Expect(ledger.Balance(20) == 525, "arbiter balance");
}
{
    var ledger = Example();
    Expect(Throws<NotAuthorizedException>(() => ledger.Release(20, 1)), "seller rejected");
    Expect(ledger.Balance(20) == 25, "seller balance");
    Expect(ledger.Escrow(1)!.Status == Status.Funded && ledger.Escrow(1)!.Amount == 500, "seller state");
}
{
    var ledger = Example();
    ledger.Release(10, 1);
    Expect(Throws<BadStatusException>(() => ledger.Release(10, 1)), "second release");
    Expect(ledger.Balance(20) == 525, "credited once");
}
{
    var ledger = Example();
    Expect(Throws<NotFoundException>(() => ledger.Release(10, 99)), "missing");
    Expect(ledger.Balance(20) == 25, "missing balance");
}
{
    var ledger = Example();
    Expect(Throws<BadStatusException>(() => ledger.Release(10, 2)), "refunded");
    Expect(ledger.Balance(20) == 25, "refunded balance");
    Expect(ledger.Escrow(2)!.Amount == 100, "refunded amount");
}
{
    var ledger = Example();
    ledger.SetBalance(20, long.MaxValue - 10);
    Expect(Throws<OverflowException>(() => ledger.Release(10, 1)), "overflow");
    Expect(ledger.Balance(20) == long.MaxValue - 10, "overflow balance");
    Expect(ledger.Escrow(1)!.Status == Status.Funded && ledger.Escrow(1)!.Amount == 500, "overflow state");
}

if (failures != 0)
{
    Console.Error.WriteLine(failures + " failed");
    return 1;
}

Console.WriteLine("ok");
return 0;
