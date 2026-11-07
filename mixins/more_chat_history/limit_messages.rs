// Replaces the vanilla 100-message cap in Chat::add_message with 10,000.
fn limit_messages(value: usize) -> usize {
    let _ = value;
    10_000
}
