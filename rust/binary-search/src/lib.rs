use std::cmp::Ordering::{Equal, Greater, Less};

pub fn find<T: Ord, A: AsRef<[T]>>(array: A, key: T) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while right >= left {
        let current = left + (right - left) / 2;

        match array[current].cmp(&key) {
            Equal => return Some(current),
            Greater => {
                if current == 0 {
                    return None;
                }
                right = current - 1;
            }
            Less => left = current + 1,
        }
    }

    None
}
