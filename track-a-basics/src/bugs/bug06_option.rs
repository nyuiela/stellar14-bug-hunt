//! Bug 06: look up a balance by address.
//!
//! Build it:  cargo test -p wallet-basics --features bug06 bug06

use crate::types::Wallet;

/// Finds a wallet by its address, if we know it.
pub fn find_wallet<'a>(wallets: &'a [Wallet], address: &str) -> Option<&'a Wallet> {
    wallets.iter().find(|w| w.address == address)
}

/// The balance for an address, or 0 if we have never seen that address.
pub fn balance_of(wallets: &[Wallet], address: &str) -> u64 {
    find_wallet(wallets, address).map_or(0, |w| w.balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug06_balance_of() {
        let wallets = vec![Wallet::new("ama", "GAMA", 700)];
        assert_eq!(balance_of(&wallets, "GAMA"), 700);
        assert_eq!(balance_of(&wallets, "GNOBODY"), 0);
    }
}
