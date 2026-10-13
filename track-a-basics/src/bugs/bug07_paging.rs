//! Bug 07: page through payment history.
//!
//! Run it:  cargo test -p wallet-basics bug07

/// Returns one page of items. Pages start at 0, so page 0 is the first `page_size` items.
pub fn page<T: Clone>(items: &[T], page: usize, page_size: usize) -> Vec<T> {
    let start = page * page_size;
    items.iter().skip(start).take(page_size).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug07_first_page_starts_at_the_first_item() {
        let history: Vec<u32> = (1..=10).collect();
        assert_eq!(page(&history, 0, 3), vec![1, 2, 3]);
        assert_eq!(page(&history, 1, 3), vec![4, 5, 6]);
    }

    #[test]
    fn bug07_last_page_and_past_the_end() {
        let history: Vec<u32> = (1..=10).collect();
        assert_eq!(page(&history, 3, 3), vec![10]);
        assert!(page(&history, 4, 3).is_empty());
    }
}
