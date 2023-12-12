use std::{fs, collections::{hash_map, HashMap}};


fn main() {
    let file_path = "input2.txt";

    let factor = 1_000_000; //2 or 1_000_000 or 999_999

    let input_str = fs::read_to_string(file_path)
        .expect("Fehler beim Lesen der Datei");

    let mut grid: Vec<Vec<char>> = input_str
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();


    let empty_columns = find_empty_columns(grid.clone());

    let empty_rows = find_empty_rows(grid.clone());

    let new_grid = replace_hashtag_with_running_number(grid);

    let numbers = get_all_numbers(new_grid.clone());

    let pairs = create_unique_pair_of_numbers(numbers);

    let hash_map = pairs
    .iter()
    .map(|&pair| (pair, get_pair_coordinates(new_grid.clone(), pair.0, pair.1)))
    .collect::<hash_map::HashMap<_, _>>();    

    // println!("pairs: {:?}", pairs);

    let mut all_steps: Vec<usize> = Vec::new();

    for pair in hash_map {

        let steps = distance(pair.1.0, pair.1.1, factor, empty_rows.clone(), empty_columns.clone());
        //println!("steps: {}", steps);
        all_steps.push(steps);
    }

    let sum_of_steps: usize = all_steps.iter().sum();

    println!("sum_of_steps: {}", sum_of_steps);

}

fn distance(
    g1: (usize, usize),
    g2: (usize, usize),
    factor: usize,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
) -> usize {
    let (m1, n1) = (g1.0.min(g2.0), g1.1.min(g2.1));
    let (m2, n2) = (g1.0.max(g2.0), g1.1.max(g2.1));

    // Empty rows between g1 and g2
    let er = (m1..m2)
        .filter(|i| empty_rows.contains(&i))
        .collect::<Vec<usize>>()
        .len();

    // Empty cols between g1 and g2
    let ec = (n1..n2)
        .filter(|j| empty_cols.contains(&j))
        .collect::<Vec<usize>>()
        .len();

    let nr = (m2 - m1) - er;
    let nc = (n2 - n1) - ec;

    nr + nc + (factor * (er + ec))
}

fn get_steps(pos_a: (usize, usize), pos_b: (usize, usize)) -> i64 {

    let mut result = 0;

    let row_steps = i64::abs(pos_a.0 as i64 - pos_b.0 as i64);

    let column_steps = i64::abs(pos_a.1 as i64 - pos_b.1 as i64);

    result =  row_steps + column_steps ;

    result
}

fn get_pair_coordinates(grid: Vec<Vec<String>>, number_a: usize, number_b: usize) -> ((usize, usize), (usize, usize)) {

    let mut pos_a = get_position_of_number(grid.clone(), number_a);

    let mut pos_b = get_position_of_number(grid.clone(), number_b);

    return (pos_a, pos_b);
    
}

fn get_position_of_number(grid: Vec<Vec<String>>, number: usize) -> (usize, usize) {
    let mut result = (0, 0);

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, c) in row.iter().enumerate() {
            if let Some(digit) = to_digit_new(c.to_string()) {
                if digit as usize == number {
                    result = (row_index, col_index);
                }
            }
        }
    }

    result
}

fn to_digit_new(c: String) -> Option<usize> {
    c.parse().ok()
}

fn get_all_numbers(grid: Vec<Vec<String>>) -> Vec<usize> {
    let mut numbers: Vec<usize> = Vec::new();

    for row in &grid {
        for c in row {
            
            if let Some(digit) = to_digit_new(c.to_string()) {
                numbers.push(digit as usize);
            }
        }
    }

    numbers
}

fn create_unique_pair_of_numbers(numbers: Vec<usize>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (index, &number) in numbers.iter().enumerate() {
        for &other_number in &numbers[index + 1..] {
            result.push((number, other_number));
        }
    }

    result
}

fn count_empty_columns(grid: Vec<Vec<char>>) -> usize {
    let mut counter = 0;

    for row in &grid {
        for &c in row {
            if c == '.' {
                counter = counter + 1;
            }
        }
    }

    counter
}

fn replace_hashtag_with_running_number(grid: Vec<Vec<char>>) -> Vec<Vec<String>> {

    let mut new_grid: Vec<Vec<String>> = grid.clone().iter().map(|row| row.iter().map(|&c| c.to_string()).collect()).collect();

    let mut counter = 1;

    for row in &mut new_grid {
        for c in row {
            if *c == "#" {
                *c = counter.to_string();
                counter = counter + 1;
            }
        }
    }

    new_grid
}

fn find_empty_rows(grid: Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows: Vec<usize> = Vec::new();

    for (index, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(index);
        }
    }

    empty_rows
}


fn find_empty_columns(grid: Vec<Vec<char>>) -> Vec<usize> {

    let mut result_columns = Vec::new();

    if let Some(row) = grid.get(0) {
        for (column, &value) in row.iter().enumerate() {
            if value == '.' && grid.iter().all(|r| r[column] == '.') {
                result_columns.push(column);
            }
        }
    }

    result_columns
}

// don't need them anymore because of the new solution

fn add_empty_line_in_grid_on_index(grid: Vec<Vec<char>>, index: usize) -> Vec<Vec<char>> {

    let mut new_grid = grid.clone();

    let mut empty_line: Vec<char> = Vec::new();

    for _ in 0..grid[0].len() {
        empty_line.push('.');
    }

    new_grid.insert(index, empty_line);

    return new_grid;
    
}

fn add_empty_column_in_grid_on_index(grid: Vec<Vec<char>>, index: usize) -> Vec<Vec<char>> {

    let mut new_grid = grid.clone();

    for row in &mut new_grid {
        row.insert(index, '.');
    }

    return new_grid;
}
