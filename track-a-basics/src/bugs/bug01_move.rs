//! Bug 01: archive a payment, then describe it.
//!
//! Build it:  cargo test -p wallet-basics --features bug01 bug01

use crate::types::Payment;

/// Moves the payment into the archive and returns a one line description,
/// for example "250 stroops from GALICE to GBOB".
pub fn archive_and_describe(payment: Payment, archive: &mut Vec<Payment>) -> String {
    let line = format!("{} stroops from {} to {}", payment.amount, payment.from, payment.to);
    archive.push(payment);
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug01_archive_and_describe() {
        let mut archive = Vec::new();
        let line = archive_and_describe(Payment::native("GALICE", "GBOB", 250), &mut archive);
        assert_eq!(line, "250 stroops from GALICE to GBOB");
        assert_eq!(archive.len(), 1);
        assert_eq!(archive[0].amount, 250);
    }
}
