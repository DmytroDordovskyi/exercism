use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|factor| **factor > 0)
        .fold(HashSet::new(), |mut acc, factor| {
            let quantity = (limit - 1) / factor;
            for i in 1..=quantity {
                acc.insert(i * factor);
            }

            acc
        })
        .into_iter()
        .sum()
}
