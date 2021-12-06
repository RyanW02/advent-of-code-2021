use std::fs;

fn main() {
    let mut population = parse_input();
    for i in 0..80 {
        println!("Day {}: {} fish", i, population.len());
        let new_fish = population.iter_mut()
            .map(|fish| fish.tick())
            .filter(|x| *x)
            .count();

        for _ in 0..new_fish {
            population.push(LanternFish::new());
        }
    }
    
    println!("Fish count: {}", population.len());
}

fn parse_input() -> Vec<LanternFish> {
    let raw = fs::read_to_string("input.txt").expect("Failed to read input");
    raw.replace("\n", "").split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .map(|timer| LanternFish::new_with_timer(timer))
        .collect()
}

struct LanternFish {
    timer: u8,
}

impl LanternFish {
    pub fn new() -> LanternFish {
        LanternFish { timer: 8 }
    }

    pub fn new_with_timer(timer: u8) -> LanternFish {
        LanternFish { timer }
    }

    // returns whether to spawn a new fish
    pub fn tick(&mut self) -> bool {
        if self.timer == 0 {
            self.timer = 6;
            true
        } else {
            self.timer -= 1;
            false
        }
    }
}