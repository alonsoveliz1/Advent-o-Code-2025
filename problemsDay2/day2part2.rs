use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file")
        .trim()
        .to_string();
    // println!("{}", contents);

    let ids: Vec<String> = parse_contents(contents);
    //println!("{}", positions[0]);
    let mut counter: i64 = 0;

    for elem in ids {
        let parts: Vec<i64> = elem.split('-').map(|s| s.parse().unwrap()).collect();
        let bottom = parts[0];
        let top = parts[1];

        for nums in bottom..=top {
            if is_repeated(nums) {
                counter += nums;
            }
        }
    }

    println!("Counter: {}", counter);
}

fn parse_contents(contents: String) -> Vec<String> {
    let array: Vec<String> = contents.split(',').map(|s| s.to_string()).collect();
    return array;
}

fn is_repeated(num: i64) -> bool {
    let s = num.to_string();
    let len = s.len();

    for i in 1..=len / 2 {
        if len % i == 0 {
            let unit = &s[..i];
            let repeated = unit.repeat(len / i);
            if s == repeated {
                return true;
            }
        }
    }
    false
}
