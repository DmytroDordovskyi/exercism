pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let length = digits.len();

    digits.iter().fold(0, |acc, v| acc + v.pow(length as u32)) == num
}

fn digits(mut num: u32) -> Vec<u32> {
    let mut result = vec![];

    while num > 0 {
        result.push(num % 10);
        num /= 10;
    }

    result
}
