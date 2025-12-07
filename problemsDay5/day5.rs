use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let ranges = parse_ranges(&parts);
    let ids = parse_ids(&parts);

    let solution = solve_normal(ranges, ids);
    println!("Solution: {}", solution);
}

fn parse_ranges(contents: &[&str]) -> Vec<(u64, u64)> {
    contents[0]
        .lines()
        .map(|line| {
            let nums: Vec<u64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect()
}

fn parse_ids(contents: &[&str]) -> Vec<u64> {
    contents[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve_normal(ranges: Vec<(u64, u64)>, ids: Vec<u64>) -> u64 {
    let mut counter = 0;
    for id in ids {
        for (a, b) in &ranges {
            if *a <= id && id <= *b {
                counter += 1;
                break;
            }
        }
    }
    counter
}
