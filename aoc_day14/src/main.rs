use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let mut patterns: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Total: {:?}", tilt_north(patterns.clone()));
}


fn tilt_north(grid: Vec<Vec<char>> ) -> i64 {
    let mut total: i64 = 0;
    for col in 0..grid[0].len() {
        let mut next = 0;
        for row in 0..grid.len() {
            match grid[row][col] {
                'O' => {
                    total += grid.len() as i64 - next;
                    next += 1;
                }
                '#' => next = row as i64 + 1,
                _ => {}
            }
        }
    }

    total
}

// rotates grid right 90 degrees
fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid[0].len()];
    for col in 0..grid[0].len() {
        for row in 0..grid.len() {
            result[col][grid[0].len() - 1 - row] = grid[row][col];
        }
    }
    result
}