//! Stellar 14 bug hunt, Track B: a tiny tip jar contract.
//!
//! People deposit into their own tip balance, then tip other builders from it.
//! Every bug lives in its own file, so you can fix one without touching the others:
//!   Bug 13  src/lib.rs      tip
//!   Bug 14  src/storage.rs  read_balance
//!   Bug 15  src/checks.rs   ensure_can_tip
//!   Bug 16  src/events.rs   tipped
#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

mod checks;
mod events;
mod storage;

#[cfg(test)]
mod test;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    InvalidAmount = 1,
    InsufficientBalance = 2,
}

#[contract]
pub struct TipJar;

#[contractimpl]
impl TipJar {
    /// Adds `amount` to the caller's own tip balance and returns the new balance.
    pub fn deposit(env: Env, user: Address, amount: i128) -> Result<i128, Error> {
        user.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let new_balance = storage::read_balance(&env, &user) + amount;
        storage::write_balance(&env, &user, new_balance);
        Ok(new_balance)
    }

    /// Moves `amount` from `from` to `to`. Only `from` may approve this.
    pub fn tip(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        from.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        let from_balance = storage::read_balance(&env, &from);
        checks::ensure_can_tip(from_balance, amount)?;

        storage::write_balance(&env, &from, from_balance - amount);
        let to_balance = storage::read_balance(&env, &to);
        storage::write_balance(&env, &to, to_balance + amount);

        events::tipped(&env, &from, &to, amount);
        Ok(())
    }

    /// The tip balance for `user`. Anyone can read it.
    pub fn balance(env: Env, user: Address) -> i128 {
        storage::read_balance(&env, &user)
    }
}
