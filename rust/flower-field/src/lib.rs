pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut garden: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();

    for i in 0..garden.len() {
        for j in 0..garden[i].len() {
            if garden[i][j] == ' ' {
                let star_neighbors = find_star_neighbors(&garden, i, j);
                if star_neighbors > 0 {
                    garden[i][j] = char::from_digit(star_neighbors as u32, 10).unwrap();
                }
            }
        }
    }

    garden
        .into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn find_star_neighbors(garden: &[Vec<char>], i: usize, j: usize) -> usize {
    if garden.is_empty() {
        return 0;
    }

    let mut count = 0;
    let rows = garden.len();
    let cols = garden[0].len();

    if i > 0 && j > 0 && garden[i - 1][j - 1] == '*' {
        count += 1;
    }
    if i > 0 && garden[i - 1][j] == '*' {
        count += 1;
    }
    if i > 0 && j + 1 < cols && garden[i - 1][j + 1] == '*' {
        count += 1;
    }
    if j > 0 && garden[i][j - 1] == '*' {
        count += 1;
    }
    if j + 1 < cols && garden[i][j + 1] == '*' {
        count += 1;
    }
    if i + 1 < rows && j > 0 && garden[i + 1][j - 1] == '*' {
        count += 1;
    }
    if i + 1 < rows && garden[i + 1][j] == '*' {
        count += 1;
    }
    if i + 1 < rows && j + 1 < cols && garden[i + 1][j + 1] == '*' {
        count += 1;
    }

    count
}
