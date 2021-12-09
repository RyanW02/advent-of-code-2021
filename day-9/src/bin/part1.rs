use std::fs;

fn main() {
    let grid = read_input();

    let mut sum: usize = 0;
    for (row_index, row) in grid.iter().enumerate() {
        'inner:
        for i in 0..row.len() {
            // Compare value to the left
            if i > 0 {
                if row[i] >= row[i - 1] {
                    continue 'inner;
                }
            }

            // Compare value to the right
            if i < row.len() - 1 {
                if row[i] >= row[i + 1] {
                    continue 'inner;
                }
            }

            // Compare value above
            if row_index > 0 {
                if row[i] >= grid[row_index - 1][i] {
                    continue 'inner;
                }
            }

            // Compare value below
            if row_index < grid.len() - 1 {
                if row[i] >= grid[row_index + 1][i] {
                    continue 'inner;
                }
            }

            sum += (row[i] as usize) + 1;
        }
    }

    println!("Sum: {}", sum);
}

fn read_input() -> Vec<Vec<u8>> {
    fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}