use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use num_traits::pow;


#[derive(Debug, Clone)]
struct Scratchcard {
    card_number: i32,
    winning_numbers: Vec<i32>,
    own_numbers: Vec<i32>,
    copies: i32
}

impl Scratchcard {
    fn find_matching_numbers(&self) -> Vec<i32> {
        self.winning_numbers
            .iter()
            .filter(|&num| self.own_numbers.contains(num))
            .cloned()
            .collect()
    }

    fn calculate_points(&self) -> i32 {
        let matching_numbers = self.find_matching_numbers();

        let length = matching_numbers.len();

        if length == 0 {
            return 0;
        }

        if length == 1 {
            return 1;
        }

        let result = pow(2, length-1);

        return result;

    }
}

fn main() {
    let mut game_results: Vec<Scratchcard> = Vec::new();

    let mut more_game_results: Vec<Scratchcard> = Vec::new();

    let mut sum_result = 0;

    let mut sum_result_scratchcards = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {

                let scratchcard = parse_scratchcard(&ip);
                game_results.push(scratchcard);
            }
        }

        for game in &game_results {
            //println!("Card: {:?}", game);
            //println!("Winning Numbers: {:?}", game.winning_numbers);
            //println!("Own Numbers: {:?}", game.own_numbers);
            //println!("Calc {}:", game.calculate_points());
            //println!("Matching Numbers: {:?}", game.find_matching_numbers());
            sum_result += game.calculate_points();
        }
        
        for index in 0..game_results.len() {            
            let copie_amounts = game_results[index].find_matching_numbers().len();
            
            let multiplier = 1+game_results[index].copies;

            for index2 in 1..copie_amounts+1 {                


                // println!("Original:{} Set:{} | {} with multiplier to {} | => copies to {}", 
                //     game_results[index].card_number, 
                //     game_results[index+index2].card_number, 
                //     game_results[index+index2].copies,
                //     multiplier,
                //     game_results[index+index2].copies + multiplier);

                game_results[index+index2].copies += multiplier;
            }            
        }
    
    }

    for game in &game_results {
        let instances = game.copies+1;
        println!("Card: {:?} | Instances: {}", game.card_number, instances);
        sum_result_scratchcards += instances;
    }

    println!("Sum of all scratchcards: {}", sum_result_scratchcards);

    //println!("Sum of all points: {}", sum_result);
    //println!("Sum of all scratchcards: {:?}", game_results);
    
}



fn parse_scratchcard(input: &str) -> Scratchcard {
    let mut parts = input.splitn(2, ':');
    
    let card_info: Vec<&str> = parts.next().unwrap().split_whitespace().collect();
    let card_number: i32 = card_info[1].parse().unwrap();

    let numbers_part: Vec<&str> = parts.next().unwrap().split('|').collect();

    let winning_numbers: Vec<i32> = numbers_part[0].split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let own_numbers: Vec<i32> = numbers_part[1].split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    Scratchcard {
        card_number,
        winning_numbers,
        own_numbers,
        copies: 0
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}