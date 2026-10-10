//! Safety checks. Bug 15 is in this file.

use crate::Error;

/// A tip must never take a balance below zero.
/// Return `Error::InsufficientBalance` when `amount` is more than `balance`.
pub fn ensure_can_tip(balance: i128, amount: i128) -> Result<(), Error> {
    if amount > balance {
        return Err(Error::InsufficientBalance);
    }
    Ok(())
}
