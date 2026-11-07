//! More Chat History runtime module.
//!
//! Replaces Horizon's default 100-message / 100-line chat scrollback caps
//! with a 10,000-line capacity, allowing players to scroll much further
//! back through old chat messages and server logs.

pub const CHAT_HISTORY_LIMIT: usize = 10_000;

#[inline]
pub fn get_chat_limit(_original: usize) -> usize {
    CHAT_HISTORY_LIMIT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_limit_expands_to_10000() {
        assert_eq!(get_chat_limit(100), 10_000);
    }
}
