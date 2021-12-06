use std::fs;

fn main() {
    let mut population: [usize; 9] = [0; 9];
    for offset in parse_input() {
        population[offset as usize] += 1;
    }

    for _ in 0..256 {
        let new_fish = population[0];
        population.rotate_left(1);
        population[6] += new_fish;
        population[8] = new_fish;
    }

    let count: usize = population.iter().sum::<usize>();
    println!("Fish count: {}", count);

    /*let mut population: [usize; 7] = [0; 7];
    let mut population_first_gen: [usize; 10] = [0; 10];
    for offset in parse_input() {
        population[offset as usize] += 1;
    }

    for gen in 0..18 {
        population[6] += population_first_gen[0];
        population_first_gen[9] += population_first_gen[0];
        population_first_gen[0] = 0;
        population_first_gen[9] += population[0];

        for i in 0..population.len() {
            for j in 0..population[i]{
                print!("{},", i);
            }
        }
        
        for i in 0..population_first_gen.len() {
            for j in 0..population_first_gen[i]{
                print!("{},", i);
            }
        }

        print!("\n");


        population_first_gen.rotate_left(1);
        population.rotate_left(1);
    }

    let count: usize = population.iter().sum::<usize>() + population_first_gen.iter().sum::<usize>();
    println!("Fish count: {}", count);*/
}

fn parse_input() -> Vec<u8> {
    let raw = fs::read_to_string("input.txt").expect("Failed to read input");
    raw.replace("\n", "").split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}