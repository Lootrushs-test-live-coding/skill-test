use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Funded,
    Released,
    Refunded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Escrow {
    pub buyer: u64,
    pub seller: u64,
    pub arbiter: u64,
    pub amount: u64,
    pub status: Status,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReleaseError {
    NotFound,
    NotAuthorized,
    BadStatus,
    Overflow,
}

#[derive(Default)]
pub struct Ledger {
    balances: HashMap<u64, u64>,
    escrows: HashMap<u64, Escrow>,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put_escrow(&mut self, id: u64, escrow: Escrow) {
        self.escrows.insert(id, escrow);
    }

    pub fn set_balance(&mut self, account: u64, amount: u64) {
        self.balances.insert(account, amount);
    }

    pub fn balance(&self, account: u64) -> u64 {
        self.balances.get(&account).copied().unwrap_or(0)
    }

    pub fn escrow(&self, id: u64) -> Option<Escrow> {
        self.escrows.get(&id).copied()
    }

    pub fn release(&mut self, caller: u64, escrow_id: u64) -> Result<(), ReleaseError> {
        let _ = (caller, escrow_id);
        unimplemented!("implement release");
    }
}
