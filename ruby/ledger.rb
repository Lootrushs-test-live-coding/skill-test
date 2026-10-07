Escrow = Struct.new(:buyer, :seller, :arbiter, :amount, :status)

class NotFound < StandardError; end
class NotAuthorized < StandardError; end
class BadStatus < StandardError; end

class Ledger
  def initialize
    @escrows = {}
    @balances = {}
  end

  def put_escrow(id, escrow)
    @escrows[id] = escrow
  end

  def set_balance(account, amount)
    @balances[account] = amount
  end

  def balance(account)
    @balances[account] || 0
  end

  def escrow(id)
    @escrows[id]
  end

  def release(caller_id, escrow_id)
    # Implement this function.
    raise NotImplementedError, "implement release"
  end
end
