//! Bug 09: withdraw from a wallet without crashing.
//!
//! Run it:  cargo test -p wallet-basics bug09

use crate::types::{Wallet, WalletError};

/// Takes `amount` out of the wallet and returns the new balance.
/// If there is not enough, return `WalletError::InsufficientFunds` and leave the balance alone.
pub fn withdraw(wallet: &mut Wallet, amount: u64) -> Result<u64, WalletError> {
    if amount > wallet.balance {
        return Err(WalletError::InsufficientFunds {
            needed: amount,
            available: wallet.balance,
        });
    }
    wallet.balance -= amount;
    Ok(wallet.balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug09_withdraw_some() {
        let mut wallet = Wallet::new("ama", "GAMA", 1_000);
        assert_eq!(withdraw(&mut wallet, 400), Ok(600));
        assert_eq!(wallet.balance, 600);
    }

    #[test]
    fn bug09_withdraw_too_much() {
        let mut wallet = Wallet::new("kofi", "GKOFI", 100);
        assert_eq!(
            withdraw(&mut wallet, 500),
            Err(WalletError::InsufficientFunds {
                needed: 500,
                available: 100
            })
        );
        assert_eq!(wallet.balance, 100);
    }
}
