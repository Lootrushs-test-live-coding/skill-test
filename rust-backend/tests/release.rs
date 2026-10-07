use escrow_ledger::{Escrow, Ledger, ReleaseError, Status};

fn example() -> Ledger {
    let mut ledger = Ledger::new();
    ledger.put_escrow(
        1,
        Escrow {
            buyer: 10,
            seller: 20,
            arbiter: 30,
            amount: 500,
            status: Status::Funded,
        },
    );
    ledger.put_escrow(
        2,
        Escrow {
            buyer: 10,
            seller: 20,
            arbiter: 30,
            amount: 100,
            status: Status::Refunded,
        },
    );
    ledger.set_balance(20, 25);
    ledger
}

#[test]
fn buyer_releases() {
    let mut ledger = example();
    ledger.release(10, 1).unwrap();
    assert_eq!(ledger.balance(20), 525);
    let escrow = ledger.escrow(1).unwrap();
    assert_eq!(escrow.status, Status::Released);
    assert_eq!(escrow.amount, 0);
    assert_eq!(ledger.escrow(2).unwrap().status, Status::Refunded);
}

#[test]
fn arbiter_releases() {
    let mut ledger = example();
    ledger.release(30, 1).unwrap();
    assert_eq!(ledger.balance(20), 525);
    assert_eq!(ledger.escrow(1).unwrap().status, Status::Released);
}

#[test]
fn seller_is_rejected() {
    let mut ledger = example();
    assert_eq!(ledger.release(20, 1), Err(ReleaseError::NotAuthorized));
    assert_eq!(ledger.balance(20), 25);
    assert_eq!(ledger.escrow(1).unwrap().status, Status::Funded);
    assert_eq!(ledger.escrow(1).unwrap().amount, 500);
}

#[test]
fn second_release_is_rejected() {
    let mut ledger = example();
    ledger.release(10, 1).unwrap();
    assert_eq!(ledger.release(10, 1), Err(ReleaseError::BadStatus));
    assert_eq!(ledger.balance(20), 525);
}

#[test]
fn missing_escrow() {
    let mut ledger = example();
    assert_eq!(ledger.release(10, 99), Err(ReleaseError::NotFound));
    assert_eq!(ledger.balance(20), 25);
}

#[test]
fn refunded_escrow_is_rejected() {
    let mut ledger = example();
    assert_eq!(ledger.release(10, 2), Err(ReleaseError::BadStatus));
    assert_eq!(ledger.balance(20), 25);
    assert_eq!(ledger.escrow(2).unwrap().amount, 100);
}

#[test]
fn overflow_leaves_state() {
    let mut ledger = example();
    ledger.set_balance(20, u64::MAX - 10);
    assert_eq!(ledger.release(10, 1), Err(ReleaseError::Overflow));
    assert_eq!(ledger.balance(20), u64::MAX - 10);
    assert_eq!(ledger.escrow(1).unwrap().status, Status::Funded);
    assert_eq!(ledger.escrow(1).unwrap().amount, 500);
}
