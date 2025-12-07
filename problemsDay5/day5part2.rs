use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = contents.split("\n\n").collect();

    let ranges = parse_ranges(&parts);
    let merged_ranges = merge_ranges(ranges);

    let solution = solve_normal(merged_ranges);
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

fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(u64, u64)> = vec![];

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }
    merged
}

fn solve_normal(ranges: Vec<(u64, u64)>) -> u64 {
    let mut counter = 0;

    for (a, b) in &ranges {
        counter += *b - *a + 1;
    }

    counter
}
