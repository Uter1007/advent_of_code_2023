use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let file_path = "input.txt";

    let input_str = fs::read_to_string(file_path)
        .expect("Fehler beim Lesen der Datei");

    let mut grid: Vec<Vec<char>> = input_str
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();


    let mut s_position: Option<(usize, usize)> = None;
    for (row_index, row) in grid.iter().enumerate() {
        if let Some(col_index) = row.iter().position(|&c| c == 'S') {
            s_position = Some((row_index, col_index));
            break;
        }
    }

    println!("s_position: {:?}", s_position);

/*     for row in &grid {
        for &c in row {
            print!("{}", c);
        }
        println!();
    } */

    let mut current_position = s_position.unwrap();
    let mut last_pos = DIRECTION::RIGHT;

    let mut counter = 0;

    let mut all_pos: Vec<(usize, usize)> = Vec::new();

    all_pos.push(current_position);

    loop {
        
        (current_position, last_pos) = get_next_pos(grid.clone(), current_position, last_pos);

        all_pos.push(current_position);

        counter = counter + 1;
        let new_val = grid[current_position.0][current_position.1];

        if new_val == 'S' {
            break;
        }
    }

    println!("counter: {}", (counter / 2));

    // println!("all_pos: {:?}", all_pos);

    // replace not visited positions with '.'

    let mut new_grid = grid.clone();

    for (row_index, row) in grid.clone().iter().enumerate() {
        for (col_index, &c) in row.iter().enumerate() {

            if !all_pos.contains(&(row_index, col_index)) {
                new_grid[row_index][col_index] = '.';
            }
        }
    }

    /*# Corruption spreads like a virus within the Matrix
for y, line in enumerate(mtrx):
    in_simulated_reality = False
    for x, c in enumerate(line):
        # - flip simulated reality at |, J and L
        if c == '|' or c == 'J' or c == 'L':
            in_simulated_reality = not in_simulated_reality
        elif c == '.' and in_simulated_reality:
            mtrx[y][x] = 'I'
            insiders.append((x, y))
 */

    let mut insiders: Vec<(usize, usize)> = Vec::new();

    for (row_index, row) in new_grid.clone().iter().enumerate() {
        let mut in_simulated_reality = false;

        for (col_index, &c) in row.iter().enumerate() {
            

            if c == '|' || c == 'J' || c == 'L' {
                in_simulated_reality = !in_simulated_reality;
            } else if c == '.' && in_simulated_reality {
                new_grid[row_index][col_index] = 'I';
                insiders.push((row_index, col_index));
            }
        }
    }

    println!("new_grid: {:?}", new_grid);

    println!("insiders: {:?}", insiders.len());

}

fn get_next_pos(grid: Vec<Vec<char>>, current_position: (usize, usize), last_direction: DIRECTION) -> ((usize, usize), DIRECTION) {

    let current_pos = grid[current_position.0][current_position.1];

    // println!("right_pos: {}", right_pos);
    // println!("down_pos: {}", down_pos);
    // println!("left_pos: {}", left_pos);
    // println!("up_pos: {}", up_pos);

    if current_pos == 'S' {
        let right_pos = grid[current_position.0][current_position.1 + 1];
        if right_pos == '-' || right_pos == 'J' || right_pos == '7' {
            return ((current_position.0, current_position.1 + 1), DIRECTION::RIGHT);
        }

        let down_pos = grid[current_position.0 + 1][current_position.1];
        if down_pos == '|' || down_pos == 'L' || down_pos == 'J' {
            return ((current_position.0 + 1, current_position.1), DIRECTION::DOWN);
        }

        let up_pos: char = grid[current_position.0 - 1][current_position.1];
        if up_pos == '|' || up_pos == 'F' || up_pos == '7' {
            return ((current_position.0 - 1, current_position.1), DIRECTION::UP);
        }

        let left_pos = grid[current_position.0][current_position.1 - 1];
        if left_pos == '-' || left_pos == 'F' || left_pos == 'L' {
            return ((current_position.0, current_position.1 - 1), DIRECTION::LEFT);
        }
    }

    if last_direction == DIRECTION::DOWN {
        if current_pos == '|' {
            return ((current_position.0 + 1, current_position.1), DIRECTION::DOWN);
        }

        if current_pos == 'J' {
            return ((current_position.0, current_position.1-1), DIRECTION::LEFT);
        }

        if current_pos == 'L' {
            return ((current_position.0, current_position.1+1), DIRECTION::RIGHT);
        }
    }

    if last_direction == DIRECTION::UP {
        if current_pos == '|' {
            return ((current_position.0 - 1, current_position.1), DIRECTION::UP);
        }

        if current_pos == 'F' {
            return ((current_position.0, current_position.1+1), DIRECTION::RIGHT);
        }

        if current_pos == '7' {
            return ((current_position.0, current_position.1-1), DIRECTION::LEFT);
        }
    }

    if last_direction == DIRECTION::LEFT {
        if current_pos == '-' {
            return ((current_position.0, current_position.1 - 1), DIRECTION::LEFT);
        }

        if current_pos == 'F' {
            return ((current_position.0+1, current_position.1), DIRECTION::DOWN);
        }

        if current_pos == 'L' {
            return ((current_position.0-1, current_position.1), DIRECTION::UP);
        }
    }

    if last_direction == DIRECTION::RIGHT {
        if current_pos == '-' {
            return ((current_position.0, current_position.1 + 1), DIRECTION::RIGHT);
        }

        if current_pos == 'J' {
            return ((current_position.0-1, current_position.1), DIRECTION::UP);
        }

        if current_pos == '7' {
            return ((current_position.0+1, current_position.1), DIRECTION::DOWN);
        }
    }
   

    return ((0, 0), DIRECTION::RIGHT);


}
