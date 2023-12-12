use std::{fs, collections::{hash_map, HashMap}};

#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: u64,
    y: u64,
}
const EXPAND: u64 = 999_999; // 1 or 999_999 because we count as dist 1

fn main() {
    let file_path = "input2.txt";
    let input = fs::read_to_string(file_path)
        .expect("Fehler beim Lesen der Datei");
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    //map galaxies
    let mut coords = lines
        .iter()
        .enumerate()
        .flat_map(|(y, ln)| {
            ln.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Galaxy {
                    x: x as u64,
                    y: y as u64,
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect::<Vec<Galaxy>>();

    let x_gallaxy_coords = coords.iter().map(|c| c.x).collect::<Vec<u64>>();
    let y_galaxy_coords = coords.iter().map(|c| c.y).collect::<Vec<u64>>();

    //compute expansions x,y coord
    let x_expanded = (0..width)
        .filter_map(|y| u64::try_from(y).ok())
        .filter(|y| !x_gallaxy_coords.contains(y))
        .collect::<Vec<u64>>();
    let y_expanded = (0..height)
        .filter_map(|y| u64::try_from(y).ok())
        .filter(|y| !y_galaxy_coords.contains(y))
        .collect::<Vec<u64>>();

    let x_gt_than = |x: u64| x_expanded.iter().filter(|&&x_expand| x > x_expand).count() as u64;
    let y_gt_than = |y: u64| y_expanded.iter().filter(|&&y_expand| y > y_expand).count() as u64;

    //mutate coord positions
    coords.iter_mut().for_each(|coord| {
        coord.x += x_gt_than(coord.x) * EXPAND;
        coord.y += y_gt_than(coord.y) * EXPAND;
    });

    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (g1, g2) = (&coords[i], &coords[j]);
            sum += grid_distance(g1.x as i64, g1.y as i64, g2.x as i64, g2.y as i64);
        }
    }
    println!("RESULT: {sum}");
}

fn grid_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}