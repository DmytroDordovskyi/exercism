use std::cmp::min;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let garden: Vec<Vec<char>> = garden.iter().map(|row| row.chars().collect()).collect();

    (0..garden.len())
        .map(|i| {
            (0..garden[i].len())
                .map(|j| {
                    if garden[i][j] == ' ' {
                        let star_neighbors = find_star_neighbors(&garden, i, j);
                        if star_neighbors > 0 {
                            char::from_digit(star_neighbors as u32, 10).unwrap()
                        } else {
                            ' '
                        }
                    } else {
                        garden[i][j]
                    }
                })
                .collect()
        })
        .collect()
}

fn find_star_neighbors(garden: &[Vec<char>], i: usize, j: usize) -> usize {
    if garden.is_empty() {
        return 0;
    }

    let (i_min, i_max) = (i.saturating_sub(1), min(i + 1, garden.len() - 1));
    let (j_min, j_max) = (j.saturating_sub(1), min(j + 1, garden[0].len() - 1));
    (i_min..=i_max)
        .flat_map(|i| (j_min..=j_max).map(move |j| (i, j)))
        .filter(|&(r, c)| (r != i || c != j) && garden[r][c] == '*')
        .count()
}
