use std::fs;

fn main() {
    let lines = read_input();

    let mut total = 0;
    for line in lines {
        let mut split = line.split("|");
        let raw_outputs: Vec<&str> = split.next().unwrap().trim_end().split(" ").collect();
        let digits: Vec<&str> = split.next().unwrap().trim_start().split(" ").collect();

        let mut sum = 0;
        for (index, digit) in digits.iter().enumerate() {
            if let Some(digit) = get_digit(digit) {
                sum += digit * get_multiplier(index);
            } else {
                if digit.len() == 5 {
                    // [2,3,5]
                    // If contains format for 7, it's a 3
                    let seven = raw_outputs.iter().find(|x| x.len() == 3).unwrap();
                    if str_subset(digit, seven) {
                        sum += 3 * get_multiplier(index);
                    } else {
                        // 2 or a 5
                        // If intersects a 4 in 3 places, it's a 5
                        let four = raw_outputs.iter().find(|x| x.len() == 4).unwrap();
                        if str_subset_places(digit, four, 3) {
                            sum += 5 * get_multiplier(index);
                        } else {
                            // Else, it's a 2
                            sum += 2 * get_multiplier(index);
                        }
                    }
                } else if digit.len() == 6 {
                    // [0,6,9]
                    // If it contains a 4, it's a 9
                    let four = raw_outputs.iter().find(|x| x.len() == 4).unwrap();
                    if str_subset(digit, four) {
                        sum += 9 * get_multiplier(index);
                    } else {
                        // 0 or 6
                        // If contains format for 7 it's a 0
                        let seven = raw_outputs.iter().find(|x| x.len() == 3).unwrap();
                        if str_subset(digit, seven) {
                            sum += 0 * get_multiplier(index);
                        } else {
                            // Else, it's a 6
                            sum += 6 * get_multiplier(index);
                        }
                    }
                } else {
                    panic!("Invalid digit: {}", digit);
                }
            }
        }

        total += sum;
    }

    println!("Total: {}", total);
}

fn get_digit(s: &str) -> Option<i32> {
    match s.len() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}

fn get_multiplier(i: usize) -> i32 {
    match i {
        0 => 1000,
        1 => 100,
        2 => 10,
        3 => 1,
        _ => panic!("invalid index"),
    }
}

fn str_subset(s: &str, subset: &str) -> bool {
    str_subset_places(s, subset, subset.len())
}

fn str_subset_places(s: &str, subset: &str, places: usize) -> bool {
    subset
        .chars()
        .into_iter()
        .map(|c| s.contains(c))
        .filter(|x| *x)
        .count()
        == places
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("failed to read input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}
