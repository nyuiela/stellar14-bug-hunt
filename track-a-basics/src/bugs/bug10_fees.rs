//! Bug 10: look up the fee for an asset.
//!
//! Run it:  cargo test -p wallet-basics bug10

use std::collections::HashMap;

/// Looks up the fee for an asset code.
/// An asset that is not in the table returns `None`, so the caller can refuse it.
pub fn fee_for(table: &HashMap<String, u64>, code: &str) -> Option<u64> {
    table.get(code).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> HashMap<String, u64> {
        let mut t = HashMap::new();
        t.insert("XLM".to_string(), 100);
        t.insert("USDC".to_string(), 250);
        t
    }

    #[test]
    fn bug10_known_assets_have_a_fee() {
        assert_eq!(fee_for(&table(), "USDC"), Some(250));
    }

    #[test]
    fn bug10_unknown_asset_is_none() {
        assert_eq!(fee_for(&table(), "SCAMCOIN"), None);
    }
}
