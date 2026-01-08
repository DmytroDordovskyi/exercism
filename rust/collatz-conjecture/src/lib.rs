pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 { None } else { Some(count(n, 0)) }
}

fn count(n: u64, steps: u64) -> u64 {
    match n {
        1 => steps,
        n if n.is_multiple_of(2) => count(n / 2, steps + 1),
        _ => count(3 * n + 1, steps + 1),
    }
}
