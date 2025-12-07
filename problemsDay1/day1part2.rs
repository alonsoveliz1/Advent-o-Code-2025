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
                zero_counter = zero_counter + (dial_value + step) / 100;
                dial_value = (dial_value + step) % 100;
            }
            "L" => {
                if dial_value == 0 {
                    zero_counter += step / 100;
                } else if step >= dial_value {
                    zero_counter += 1 + ((step - dial_value) / 100).abs();
                }
                dial_value = (dial_value - step).rem_euclid(100);
            }
            _ => {
                panic!("Uknown rotation: {}", rotation);
            }
        }
    }

    println!("{}", zero_counter);
}

fn parse_contents(contents: String) -> Vec<String> {
    let array: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    return array;
}
