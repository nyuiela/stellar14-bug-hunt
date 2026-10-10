//! Bug 11: read an amount a user typed in.
//!
//! Run it:  cargo test -p wallet-basics bug11

use crate::types::WalletError;

/// Parses a whole number of stroops typed by a user, such as " 2500 ".
/// Anything that is not a whole number returns `WalletError::InvalidAmount` with the trimmed input.
pub fn parse_stroops(input: &str) -> Result<u64, WalletError> {
    let trimmed = input.trim();
    trimmed
        .parse::<u64>()
        .map_err(|_| WalletError::InvalidAmount(trimmed.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug11_parses_a_number() {
        assert_eq!(parse_stroops(" 2500 "), Ok(2500));
    }

    #[test]
    fn bug11_rejects_text() {
        assert_eq!(
            parse_stroops("abc"),
            Err(WalletError::InvalidAmount("abc".to_string()))
        );
        assert_eq!(
            parse_stroops("-5"),
            Err(WalletError::InvalidAmount("-5".to_string()))
        );
    }
}
