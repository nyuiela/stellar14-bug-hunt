//! Bug 04: show a stroop amount as XLM.
//!
//! Build it:  cargo test -p wallet-basics --features bug04 bug04

use crate::types::STROOPS_PER_XLM;

/// Converts stroops to XLM for display. 15_000_000 stroops is 1.5 XLM.
pub fn stroops_to_xlm(stroops: i64) -> f64 {
    stroops as f64 / STROOPS_PER_XLM as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug04_stroops_to_xlm() {
        assert_eq!(stroops_to_xlm(15_000_000), 1.5);
        assert_eq!(stroops_to_xlm(10_000_000), 1.0);
        assert_eq!(stroops_to_xlm(1), 0.0000001);
    }
}
