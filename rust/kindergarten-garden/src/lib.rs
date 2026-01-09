const CHILDRENS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = CHILDRENS.iter().position(|&c| c == student).unwrap();

    diagram
        .lines()
        .flat_map(|line| {
            line[index * 2..index * 2 + 2].chars().map(|c| match c {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => "",
            })
        })
        .collect()
}
