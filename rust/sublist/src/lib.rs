#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m > n => {
            if is_sub(first_list, second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m < n => {
            if is_sub(second_list, first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        _ => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}

fn is_sub(first_list: &[i32], second_list: &[i32]) -> bool {
    first_list
        .windows(second_list.len())
        .any(|w| w == second_list)
}
