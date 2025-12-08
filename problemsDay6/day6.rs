use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let numerators: Vec<Vec<i64>> = contents
        .lines()
        .take(4)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
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

    let solution = solve(numerators, operations);
    println!("Solution: {}", solution);
}

fn solve(numerators: Vec<Vec<i64>>, operations: Vec<&str>) -> i64 {
    let width = numerators[0].len();
    let height = numerators.len();

    let mut result = 0;

    for i in 0..width {
        let mut subtotal = match operations[i] {
            "*" => 1,
            _ => 0,
        };

        match operations[i] {
            "+" => {
                for j in 0..height {
                    subtotal += numerators[j][i];
                }
            }
            "*" => {
                for j in 0..height {
                    subtotal *= numerators[j][i];
                }
            }
            _ => panic!("Unknown operation"),
        }
        result += subtotal;
    }

    result
}
