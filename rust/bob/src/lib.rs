pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with("?") => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        m if m.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_yelling(message: &str) -> bool {
    (message.to_ascii_uppercase() == message) && (message.chars().any(|c| c.is_alphabetic()))
}
