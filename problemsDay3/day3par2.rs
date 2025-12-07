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
    let mut output_voltage: i64 = 0;

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

fn get_highest(string: String) -> i64 {
    let digits: Vec<i64> = string
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i64)
        .collect();

    let n = digits.len();
    let mut n_picks = 12;
    let mut top_n = 0;
    let mut pos_last_picked = 0;

    while n_picks > 0 {
        let mut max_i = 0;
        let mut max_pos = pos_last_picked;
        for i in pos_last_picked..n - n_picks + 1 {
            if digits[i] > max_i {
                max_i = digits[i];
                max_pos = i;
            }
        }
        pos_last_picked = max_pos + 1;
        n_picks -= 1;
        top_n = top_n * 10 + max_i;
    }

    top_n
}
