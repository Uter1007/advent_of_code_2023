use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;


#[derive(Debug)]
struct GameResult {
    game_number: i32,
    green: i32,
    blue: i32,
    red: i32,
}

fn main() {
    let mut game_results: Vec<GameResult> = Vec::new();

    let color_pattern = Regex::new(r"(\d+) (\w+),?").unwrap();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {

                //println!("IP: {}", ip);
                let game_number = parse_game_number(ip.as_str());
               
            
                let parts: Vec<&str> = ip.as_str().split(';').collect();

                let mut highest_red: i32 = 0;
                let mut highest_green:i32 = 0;
                let mut highest_blue:i32 = 0;

                for part in parts {
                    let mut color_counts = HashMap::new();

                    for captures in color_pattern.captures_iter(part) {
                        // Extrahiere die Anzahl und Farbe
                        let count: i32 = captures[1].parse().unwrap();
                        let color: String = captures[2].to_string();
                
                        // Füge die Anzahl zur Summe der entsprechenden Farbe hinzu
                        let entry = color_counts.entry(color).or_insert(0);
                        *entry += count;
                        
                    }

                    // println!("Color-Counts: {:?}", color_counts);   
                    // println!("Red: {}", color_counts.get("red").unwrap_or(&0));         
                    // println!("Green: {}", color_counts.get("green").unwrap_or(&0));         
                    // println!("Blue: {}", color_counts.get("blue").unwrap_or(&0));   

                    let current_red: i32 = *color_counts.get("red").unwrap_or(&0);
                    let current_green: i32 = *color_counts.get("green").unwrap_or(&0);
                    let current_blue: i32 = *color_counts.get("blue").unwrap_or(&0);

                    if current_red > highest_red {
                        highest_red = current_red;
                    }

                    if current_green > highest_green {
                        highest_green = current_green;
                    }

                    if current_blue > highest_blue {
                        highest_blue = current_blue;
                    }

                }

                let game_result = GameResult {
                    game_number: game_number,
                    green: highest_green,
                    blue: highest_blue,
                    red: highest_red,
                };       

                game_results.push(game_result);   
            }
        }

        //println!("Game-Results: {:?}", game_results)

        let possible_red = 12;
        let possible_green = 13;
        let possible_blue = 14;

        let mut sum_result = 0;
        let mut sum_result2 = 0;


        for game in game_results {
            let sum_game = game.red*game.green*game.blue;

            if game.red <= possible_red && game.green <= possible_green && game.blue <= possible_blue {
                println!("Game-Number: {}", game.game_number);
                sum_result = sum_result + game.game_number;
            }

            sum_result2 = sum_result2 + sum_game;
        }

        println!("Sum-Result: {}", sum_result);

        println!("Sum-Result2: {}", sum_result2);
    }
    
}


fn parse_game_number(line: &str) -> i32 {
    let game_regex_pattern = Regex::new(r"Game (\d+):").unwrap();
    let game_number = game_regex_pattern.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
    return game_number;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}