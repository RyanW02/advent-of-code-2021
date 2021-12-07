use std::fs;

fn main() {
    let positions = read_input();
    let max_position = *positions.iter().max().unwrap();
    let mut fuel_cost = vec![0usize; max_position + 1];

    for i in 0..=max_position {
        let mut cost = 0usize;
        for j in &positions {
            let diff = ((*j as isize) - (i as isize)).abs() as usize;
            cost += (diff * diff + diff) / 2;
        }

        fuel_cost[i] = cost;
    }

    let (pos, cost) = fuel_cost.iter()
        .enumerate()
        .min_by_key(|(_, &cost)| cost)
        .unwrap();

    println!("Position: {} Cost: {}", pos, cost);
}

fn read_input() -> Vec<usize> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input")
        .replace("\n", "")
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}