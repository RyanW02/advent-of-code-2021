use std::fs;
use day_10::BraceType;

fn main() {
    let input = read_input();
    let score: i32 = input.iter()
        .filter_map(|line| is_corrupted(line.as_str()))
        .map(|bt| bt.get_score_part_1())
        .sum();

    println!("Score: {}", score);
}

// returns the first corrupted brace type
fn is_corrupted(line: &str) -> Option<BraceType> {
    let mut stack: Vec<BraceType> = vec![];

    for c in line.chars() {
        let brace_type = BraceType::from_char(c);
        let is_open = BraceType::is_open(c);

        if is_open {
            stack.push(brace_type);
        } else {
            let last = stack.pop();
            if let Some(last) = last {
                if last != brace_type {
                    return Some(brace_type);
                }
            }
        }
    }

    None
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
