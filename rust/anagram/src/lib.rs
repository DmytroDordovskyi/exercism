use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_chars = to_sorted_chars(&word);

    possible_anagrams
        .iter()
        .copied()
        .filter(|possible_anagram| {
            let candidate = possible_anagram.to_lowercase();

            if word == candidate {
                return false;
            }

            word_chars == to_sorted_chars(&candidate)
        })
        .collect()
}

fn to_sorted_chars(word: &str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars
}
