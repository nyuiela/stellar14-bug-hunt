//! Events that wallets and indexers listen for. Bug 16 is in this file.

use soroban_sdk::{contractevent, Address, Env};

/// Published every time someone tips.
#[contractevent]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tipped {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

pub fn tipped(env: &Env, from: &Address, to: &Address, amount: i128) {
    Tipped {
        from: from.clone(),
        to: to.clone(),
        amount,
    }
    .publish(env);
}
