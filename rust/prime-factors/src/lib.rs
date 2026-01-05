pub fn factors(mut n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut factor = 2;

    while factor * factor <= n {
        while n % factor == 0 {
            result.push(factor);
            n /= factor;
        }
        factor += 1;
    }

    if n != 1 {
        result.push(n);
    }
    result
}
