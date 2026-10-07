require "minitest/autorun"
require_relative "ledger"

def example
  ledger = Ledger.new
  ledger.put_escrow(1, Escrow.new(10, 20, 30, 500, :funded))
  ledger.put_escrow(2, Escrow.new(10, 20, 30, 100, :refunded))
  ledger.set_balance(20, 25)
  ledger
end

class ReleaseTest < Minitest::Test
  def test_buyer_releases
    ledger = example
    ledger.release(10, 1)
    assert_equal 525, ledger.balance(20)
    escrow = ledger.escrow(1)
    assert_equal :released, escrow.status
    assert_equal 0, escrow.amount
    assert_equal :refunded, ledger.escrow(2).status
  end

  def test_arbiter_releases
    ledger = example
    ledger.release(30, 1)
    assert_equal 525, ledger.balance(20)
    assert_equal :released, ledger.escrow(1).status
  end

  def test_seller_is_rejected
    ledger = example
    assert_raises(NotAuthorized) { ledger.release(20, 1) }
    assert_equal 25, ledger.balance(20)
    assert_equal :funded, ledger.escrow(1).status
    assert_equal 500, ledger.escrow(1).amount
  end

  def test_second_release_is_rejected
    ledger = example
    ledger.release(10, 1)
    assert_raises(BadStatus) { ledger.release(10, 1) }
    assert_equal 525, ledger.balance(20)
  end

  def test_missing_escrow
    ledger = example
    assert_raises(NotFound) { ledger.release(10, 99) }
    assert_equal 25, ledger.balance(20)
  end

  def test_refunded_is_rejected
    ledger = example
    assert_raises(BadStatus) { ledger.release(10, 2) }
    assert_equal 25, ledger.balance(20)
    assert_equal 100, ledger.escrow(2).amount
  end
end
