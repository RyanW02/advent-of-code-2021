use std::fs;

fn main() {
    let grid = read_input();

    let mut solver = Solver::new(grid);
    solver.analyse();
    println!("Top 3 sizes multiplied: {}", solver.top_3())
}

fn read_input() -> Vec<Vec<u8>> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(usize, usize);

struct Solver {
    visited: Vec<Coordinate>,
    grid: Vec<Vec<u8>>,
    basin_sizes: Vec<usize>,
}

impl Solver {
    fn new(grid: Vec<Vec<u8>>) -> Solver {
        Solver {
            visited: Vec::with_capacity(grid.len() * grid[0].len()),
            grid,
            basin_sizes: vec![],
        }
    }

    fn has_visited(&self, coordinate: &Coordinate) -> bool {
        self.visited.contains(&coordinate)
    }

    fn get_value(&self, coordinate: &Coordinate) -> u8 {
        self.grid[coordinate.1][coordinate.0]
    }

    // Returns top 3 basin sizes multiplied together
    fn top_3(mut self) -> usize {
        self.basin_sizes.sort();
        let top_3 = &self.basin_sizes[self.basin_sizes.len()-3..self.basin_sizes.len()];

        let mut res = 1usize;
        top_3.iter().for_each(|size| res *= *size);
        res
    }

    fn analyse(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let start = Coordinate(x, y);
                let size = self.map_basin(start);
                self.basin_sizes.push(size);
            }
        }
    }

    // returns size of basin
    fn map_basin(&mut self, start: Coordinate) -> usize {
        let mut size = 0usize;

        // Check if coordinate is already part of another basin
        if self.has_visited(&start) {
            return size;
        }

        let value = self.get_value(&start);
        if value == 9 {
            return size;
        }

        size += 1;
        self.visited.push(start);

        // Check value to the left
        if start.0 > 0 {
            let left = Coordinate(start.0 - 1, start.1);
            size += self.map_basin(left);
        }

        // Check value to the right
        if start.0 < self.grid[start.1].len() - 1 {
            let right = Coordinate(start.0 + 1, start.1);
            size += self.map_basin(right);
        }

        // Check value above
        if start.1 > 0 {
            let up = Coordinate(start.0, start.1 - 1);
            size += self.map_basin(up);
        }

        // Check value below
        if start.1 < self.grid.len() - 1 {
            let down = Coordinate(start.0, start.1 + 1);
            size += self.map_basin(down);
        }

        size
    }
}