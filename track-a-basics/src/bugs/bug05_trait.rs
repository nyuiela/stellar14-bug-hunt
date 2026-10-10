//! Bug 05: every asset should be able to describe itself.
//!
//! Build it:  cargo test -p wallet-basics --features bug05 bug05

use crate::types::{Asset, Payment};

/// Anything the wallet can print in a list.
pub trait Describe {
    fn describe(&self) -> String;
}

impl Describe for Payment {
    fn describe(&self) -> String {
        format!("payment of {} stroops to {}", self.amount, self.to)
    }
}

impl Describe for Asset {
    fn describe(&self) -> String {
        match self {
            Asset::Native => "XLM (native)".to_string(),
            Asset::Credit { code, issuer } => format!("{} issued by {}", code, issuer),
        }
    }
}

/// Describes every item in a slice, one line each.
pub fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}

/// The asset list screen.
/// Native XLM shows as "XLM (native)".
/// A credit asset shows as "<code> issued by <issuer>", for example "USDC issued by GCENTRE".
pub fn asset_lines(assets: &[Asset]) -> Vec<String> {
    describe_all(assets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug05_assets_describe_themselves() {
        let assets = vec![
            Asset::Native,
            Asset::Credit {
                code: "USDC".to_string(),
                issuer: "GCENTRE".to_string(),
            },
        ];
        assert_eq!(asset_lines(&assets), vec!["XLM (native)", "USDC issued by GCENTRE"]);
    }

    #[test]
    fn bug05_payments_still_describe_themselves() {
        let payments = vec![Payment::native("GAMA", "GKOFI", 42)];
        assert_eq!(describe_all(&payments), vec!["payment of 42 stroops to GKOFI"]);
    }
}
