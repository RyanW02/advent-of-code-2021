use std::fs;
use day_10::BraceType;

fn main() {
    let input = read_input();
    let mut scores: Vec<usize> = input.iter()
        .filter_map(|line| get_closing_sequence(line.as_str()))
        .map(|seq| {
            let mut score = 0usize;
            seq.iter().for_each(|bt| {
                score *= 5;
                score += bt.get_score_part_2();
            });
            score
        })
        .collect();

    scores.sort();
    println!("Median score: {:?}", scores[scores.len() / 2]);
}

fn get_closing_sequence(line: &str) -> Option<Vec<BraceType>> {
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
                    return None;
                }
            }
        }
    }

    stack.reverse();
    Some(stack)
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
