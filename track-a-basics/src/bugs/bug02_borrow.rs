//! Bug 02: add the treasury wallet, then report who was richest before it joined.
//!
//! Build it:  cargo test -p wallet-basics --features bug02 bug02

use crate::types::Wallet;

/// Finds the wallet with the highest balance, adds an empty treasury wallet
/// to the ledger, and returns "richest before treasury: <owner>".
pub fn add_treasury_and_report(ledger: &mut Vec<Wallet>) -> String {
    let richest = ledger
        .iter()
        .max_by_key(|w| w.balance)
        .expect("ledger should not be empty")
        .owner
        .clone();
    ledger.push(Wallet::new("treasury", "GTREASURY", 0));
    format!("richest before treasury: {}", richest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug02_add_treasury_and_report() {
        let mut ledger = vec![
            Wallet::new("ama", "GAMA", 500),
            Wallet::new("kofi", "GKOFI", 900),
            Wallet::new("esi", "GESI", 100),
        ];
        let report = add_treasury_and_report(&mut ledger);
        assert_eq!(report, "richest before treasury: kofi");
        assert_eq!(ledger.len(), 4);
        assert_eq!(ledger[3].owner, "treasury");
    }
}
