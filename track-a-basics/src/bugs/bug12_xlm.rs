//! Bug 12: turn a decimal XLM amount into stroops.
//!
//! Run it:  cargo test -p wallet-basics bug12

use crate::types::{WalletError, STROOPS_PER_XLM};

/// Converts a decimal XLM string such as "1.5" into stroops.
/// One XLM is 10_000_000 stroops, so XLM has at most 7 decimal places.
pub fn xlm_to_stroops(input: &str) -> Result<u64, WalletError> {
    let invalid = || WalletError::InvalidAmount(input.to_string());
    let (whole, frac) = match input.split_once('.') {
        Some((whole, frac)) => (whole, frac),
        None => (input, ""),
    };
    if frac.len() > 7 {
        return Err(invalid());
    }
    let whole: u64 = whole.parse().map_err(|_| invalid())?;
    let frac: u64 = if frac.is_empty() {
        0
    } else {
        format!("{:0<7}", frac).parse().map_err(|_| invalid())?
    };
    Ok(whole * STROOPS_PER_XLM as u64 + frac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug12_whole_and_tiny_amounts() {
        assert_eq!(xlm_to_stroops("2"), Ok(20_000_000));
        assert_eq!(xlm_to_stroops("0.0000001"), Ok(1));
    }

    #[test]
    fn bug12_decimal_amounts() {
        assert_eq!(xlm_to_stroops("1.5"), Ok(15_000_000));
        assert_eq!(xlm_to_stroops("12.25"), Ok(122_500_000));
    }

    #[test]
    fn bug12_too_many_decimals() {
        assert!(xlm_to_stroops("1.12345678").is_err());
    }
}
