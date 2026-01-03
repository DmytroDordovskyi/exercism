pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (start_bottles - take_down + 1..=start_bottles)
        .map(couplet)
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

fn couplet(num: u32) -> String {
    let start_line = format!("{} hanging on the wall,", start(num));
    format!(
        "{}\n{}\nAnd if one green bottle should accidentally fall,\nThere'll be {} hanging on the wall.\n",
        start_line,
        start_line,
        outcome(num)
    )
}

fn start(start_bottles: u32) -> String {
    format!(
        "{} green bottle{}",
        num_to_word(start_bottles),
        if start_bottles != 1 { "s" } else { "" }
    )
}

fn outcome(start_bottles: u32) -> String {
    format!(
        "{} green bottle{}",
        num_to_word(start_bottles - 1).to_ascii_lowercase(),
        if start_bottles != 2 { "s" } else { "" }
    )
}

fn num_to_word(num: u32) -> &'static str {
    match num {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        0 => "no",
        _ => "",
    }
}
