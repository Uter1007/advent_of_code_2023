use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Token {
    line_number: i32,
    result_as_number: i32,
    position: (usize, usize),
    token: String,
    is_symbol: bool,

}

fn main() {
    let mut endresult:i32 = 0;
    let mut endresult2:i32 = 0;
    let re = Regex::new(r"[0-9]+|[+@#&*$=\/%\-]+").unwrap();
    let mut game_results: Vec<Token> = Vec::new();
    let mut line_num = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                //println!("{}", ip);

                let matches: Vec<_> = re.find_iter(ip.as_str()).collect();

                for m in matches {
                    let token = &ip[m.start()..m.end()];
                    if !token.is_empty() {
                        let position = (m.start(), m.end());
                        // println!("Gefundenes Token: {} (Position: {:?})", token, position);
                        
                        if token.chars().all(char::is_numeric) {
                            let token_as_number = token.parse::<i32>();

                            let game_result = Token {
                                line_number: line_num,
                                result_as_number: token_as_number.unwrap(),
                                position: position,
                                token: token.to_string(),
                                is_symbol: false,
                            };       

                            game_results.push(game_result);   

                        } else {
                            let game_result = Token {
                                line_number: line_num,
                                result_as_number: 0,
                                position: position,
                                token: token.to_string(),
                                is_symbol: true,
                            };       

                            game_results.push(game_result); 
                        };                
                    }
                }
                line_num += 1;
            }
        }

        // println!("Game-Results: {:?}", game_results);

        for game in &game_results {

            let mut was_found = false;

            if !game.is_symbol {

                let current_line_number = game.line_number;
                let mut current_min_position = game.position.0;
                let mut current_max_position = game.position.1;
                
                if current_min_position > 0 {
                    current_min_position -= 1;
                }

                current_max_position += 1;

                let mut current_line_number_min = game.line_number;
                let mut current_line_number_max = game.line_number;

                if current_line_number_min > 0 {
                    current_line_number_min -= 1;
                }

                current_line_number_max += 1;

                //println!("Game: {:?} => L:{}-{} C:{}-{}", game.token, current_line_number_min, current_line_number_max, current_min_position, current_max_position);

                if let Some(found_token) = game_results.iter().find(|&token| {
                    token.position.0 >= current_min_position
                    && token.position.1 <= current_max_position
                    && token.is_symbol
                    && token.line_number <= current_line_number_max
                    && token.line_number >= current_line_number_min
                }) {
                    //println!("Game: {:?} => Gefundenes Token: {} (Position: {:?})", game ,found_token.token, found_token.position);
                    was_found = true;
                } else {
                    /* 
                    println!("Kein passendes Token gefunden. {} (Position: {:?}) | {} {} {}", 
                    game.token, game.position, current_line_number, current_min_position, current_max_position);
                    */
                }

                if was_found {
                    endresult += game.result_as_number;
                }
                
            }
        }

        println!("Endresult: {}", endresult);

        for game in &game_results {
            let mut was_found = false;

            if game.is_symbol && game.token == "*" {

                println!("Game: {:?} => L:{} C:{}-{}", game.token, game.line_number, game.position.0, game.position.1);

                let current_line_number = game.line_number;
                let mut current_min_position = game.position.0;
                let mut current_max_position = game.position.1;

                let mut current_line_number_min = game.line_number;
                let mut current_line_number_max = game.line_number;

                if current_line_number_min > 0 {
                    current_line_number_min -= 1;
                }

                current_line_number_max += 1;

                // println!("Game: {:?} => L:{}-{} C:{}-{}", game.token, current_line_number_min, current_line_number_max, current_min_position, current_max_position);

                let found_tokens: Vec<_> = game_results
                    .iter()
                    .filter(|&token| {
                        (token.position.0 <= current_max_position && token.position.1 >= current_min_position)
                        && token.is_symbol == false
                        && token.line_number <= current_line_number_max
                        && token.line_number >= current_line_number_min
                    })
                    .collect();

                if found_tokens.len() == 2 { 
                   //println!("Game: {:?} => Gefundene Tokens: {:?}", game ,found_tokens);
                   let result = found_tokens[0].result_as_number * found_tokens[1].result_as_number;
                   endresult2 += result;
                }
            }
        }

        println!("Endresult2: {}", endresult2);

    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}