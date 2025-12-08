use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let max_width: i64 = contents.lines().map(|line| line.len()).max().unwrap() as i64;
    let numerators: Vec<Vec<char>> = contents
        .lines()
        .take(4)
        .map(|line| line.chars().collect())
        .collect();

    let operations: Vec<&str> = contents
        .lines()
        .nth(4)
        .unwrap()
        .split_whitespace()
        .collect();

    for s in &operations {
        println!("{}", s);
    }

    let solution = solve(numerators, operations, max_width);
    println!("Solution: {}", solution);
}

fn solve(numerators: Vec<Vec<char>>, operations: Vec<&str>, max_width: i64) -> i64 {
    let width = numerators[0].len();
    let height = numerators.len();

    let mut result = 0;
    let mut col: isize = (max_width - 1) as isize;
    let mut n_op = operations.len();

    while col >= 0 {
        if is_all_spaces(&numerators, col, height) {
            col -= 1;
            continue;
        }
        n_op -= 1;
        let mut numbers: Vec<i64> = Vec::new();

        while col >= 0 && !is_all_spaces(&numerators, col, height) {
            let mut num: i64 = 0;
            for row in 0..height {
                let c = numerators[row][col as usize];
                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap() as i64;
                }
            }
            numbers.push(num);
            col -= 1;
        }

        let subtotal: i64 = match operations[n_op] {
            "+" => numbers.iter().sum(),
            "*" => numbers.iter().product(),
            _ => panic!("Unknown function"),
        };
        println!("Subtotal {}", subtotal);
        result += subtotal;
    }
    result
}

fn is_all_spaces(grid: &Vec<Vec<char>>, col: isize, height: usize) -> bool {
    (0..height).all(|row| grid[row][col as usize] == ' ')
}
