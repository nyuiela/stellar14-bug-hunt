//! Bug 03: a friendly label for every transaction status.
//!
//! Build it:  cargo test -p wallet-basics --features bug03 bug03

use crate::types::TxStatus;

/// The text the wallet shows next to a transaction.
/// An expired transaction should say "expired, please resubmit".
pub fn status_label(status: TxStatus) -> &'static str {
    match status {
        TxStatus::Pending => "waiting for the network",
        TxStatus::Success => "confirmed",
        TxStatus::Failed => "rejected",
        TxStatus::Expired => "expired, please resubmit",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug03_every_status_has_a_label() {
        assert_eq!(status_label(TxStatus::Pending), "waiting for the network");
        assert_eq!(status_label(TxStatus::Success), "confirmed");
        assert_eq!(status_label(TxStatus::Failed), "rejected");
        assert_eq!(status_label(TxStatus::Expired), "expired, please resubmit");
    }
}
