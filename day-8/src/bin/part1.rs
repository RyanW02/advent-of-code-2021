use std::fs;

fn main() {
    let lines = read_input();

    let mut count = 0;
    for line in lines {
        let digits = line.split("| ").nth(1).unwrap();
        count += digits.split(" ").filter_map(|s| get_digit(s)).count();
    }

    println!("Digits 1|4|7|8: {}", count);
}

fn get_digit(s: &str) -> Option<i32> {
    match s.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None
    }
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("failed to read input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}