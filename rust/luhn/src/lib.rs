/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !is_valid_format(code) {
        return false;
    }

    let digits = str_to_digits(code);

    if digits.len() < 2 {
        return false;
    }

    let verification_sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(ind, value)| {
            if ind % 2 == 1 {
                let double = value * 2;
                if double > 9 { double - 9 } else { double }
            } else {
                *value
            }
        })
        .sum();
    verification_sum.is_multiple_of(10)
}

fn is_valid_format(code: &str) -> bool {
    code.chars()
        .all(|c| c.is_whitespace() || c.is_ascii_digit())
}

fn str_to_digits(code: &str) -> Vec<u32> {
    code.chars().filter_map(|c| c.to_digit(10)).collect()
}
