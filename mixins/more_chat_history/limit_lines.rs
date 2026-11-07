// Replaces the vanilla 100-line cap in Chat::add_lines with 10,000.
fn limit_lines(value: usize) -> usize {
    let _ = value;
    10_000
}
