use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string();
    // println!("{}", contents);

    let batteries: Vec<String> = parse_contents(contents);
    //println!("{}", positions[0]);
    let mut output_voltage: i32 = 0;

    for battery in batteries {
        println!("Battery: {}", battery);
        let voltage = get_highest(battery);
        println!("Voltage: {}", voltage);
        output_voltage += voltage;
    }

    println!("Counter: {}", output_voltage);
}

fn parse_contents(contents: String) -> Vec<String> {
    contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn get_highest(string: String) -> i32 {
    let digits: Vec<i32> = string
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();

    let n = digits.len();

    let mut max_from_right: Vec<i32> = vec![0; n];
    max_from_right[n - 1] = digits[n - 1];

    for i in (0..n - 1).rev() {
        max_from_right[i] = max_from_right[i + 1].max(digits[i]);
    }

    let mut best = 0;
    for i in 0..n - 1 {
        let combined = concat_integers(digits[i], max_from_right[i + 1]);
        best = best.max(combined);
    }
    best
}

fn concat_integers(a: i32, b: i32) -> i32 {
    format!("{}{}", a, b).parse().unwrap()
}
