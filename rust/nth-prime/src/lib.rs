pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut i: u32 = 1;

    while primes.len() <= n as usize {
        add_if_prime(6 * i - 1, &mut primes);
        add_if_prime(6 * i + 1, &mut primes);

        i += 1;
    }

    primes[n as usize]
}

fn add_if_prime(n: u32, primes: &mut Vec<u32>) {
    let limit = (n as f64).sqrt() as u32;

    if !primes
        .iter()
        .take_while(|&&prime| prime <= limit)
        .any(|prime| n % prime == 0)
    {
        primes.push(n);
    }
}
