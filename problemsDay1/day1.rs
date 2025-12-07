use std::env;
use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    // println!("{}", contents);

    let rotations: Vec<String> = parse_contents(contents);
    //println!("{}", positions[0]);

    let mut dial_value = 50;
    let mut zero_counter = 0;

    for elem in rotations {
        let rotation = &elem[0..1];
        let step: i32 = elem[1..].parse().unwrap();

        println!("Rotation: {}, steps: {}", rotation, step);
        match rotation {
            "R" => {
                dial_value = (dial_value + step) % 100;
            }
            "L" => {
                dial_value = (dial_value - step) % 100;
            }
            _ => {
                panic!("Uknown rotation: {}", rotation);
            }
        }

        if dial_value == 0 {
            zero_counter = zero_counter + 1;
        }
    }

    println!("{}", zero_counter);
}

fn parse_contents(contents: String) -> Vec<String> {
    let array: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    return array;
}
