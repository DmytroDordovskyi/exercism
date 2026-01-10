pub fn abbreviate(phrase: &str) -> String {
    format!(" {phrase}")
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|chs| {
            (!chs[0].is_alphabetic() && chs[0] != '\'' && chs[1].is_alphabetic())
                || (chs[0].is_alphabetic()
                    && chs[0].is_ascii_lowercase()
                    && chs[1].is_alphabetic()
                    && chs[1].is_ascii_uppercase())
        })
        .map(|chs| chs[1].to_ascii_uppercase())
        .collect()
}
