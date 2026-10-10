//! Where balances live. Bug 14 is in this file.

use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address),
}

/// Reads a balance. A user we have never seen has a balance of 0.
pub fn read_balance(env: &Env, user: &Address) -> i128 {
    env.storage()
        .persistent()
        .get(&DataKey::Balance(user.clone()))
        .unwrap_or(0)
}

/// Saves a balance in persistent storage, one entry per user.
pub fn write_balance(env: &Env, user: &Address, amount: i128) {
    env.storage()
        .persistent()
        .set(&DataKey::Balance(user.clone()), &amount);
}
