use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let array: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let solution = solve(array);
    println!("Solution: {}", solution);
}

fn solve(array: Vec<Vec<char>>) -> i64 {
    let mut width = array[0].len();
    let mut rays = vec![0; width];
    let mut solution = 0;

    for rows in array {
        for (i, pos) in rows.iter().enumerate() {
            if *pos == 'S' {
                rays[i] = 1;
            }
            if *pos == '^' {
                if rays[i] == 1 {
                    rays[i - 1] = 1;
                    rays[i + 1] = 1;
                    rays[i] = 0;
                    solution += 1;
                }
            }
        }
    }
    solution
}
