use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string();

    let positions: Vec<Vec<char>> = parse_contents(contents);

    let solution = solve(positions);
    println!("Number of rolls: {}", solution);
}

fn parse_contents(contents: String) -> Vec<Vec<char>> {
    contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect())
        .collect()
}

fn solve(mut array: Vec<Vec<char>>) -> i32 {
    let mut n = 0;

    let height = array.len();
    let width = array[0].len();

    let mut removed = true;
    while removed {
        let mut removed_i = false;
        for i in 0..height {
            for j in 0..width {
                if array[i][j] == '@' {
                    let adjacent = check_adjacent_rolls(&array, i, j);
                    if adjacent < 4 {
                        n += 1;
                        array[i][j] = '.';
                        removed_i = true;
                    }
                }
            }
        }
        if !removed_i {
            removed = false;
        }
    }
    n
}

fn check_adjacent_rolls(array: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut count: i32 = 0;
    for i in x.saturating_sub(1)..=(x + 1).min(array.len() - 1) {
        for j in y.saturating_sub(1)..=(y + 1).min(array[0].len() - 1) {
            if i == x && j == y {
                continue;
            }
            if array[i][j] == '@' {
                count += 1;
            }
        }
    }
    count
}
