use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct ProgramLine {
    instructions: Vec<usize>,
    lines: String
}


fn main() {
    if let Ok(lines) = read_lines("./input2.txt") {
        let mut program_lines: Vec<ProgramLine> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                
                let parts = ip.split(" ")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();

                let instructions = parts[1].split(",")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                program_lines.push(ProgramLine {
                    instructions: instructions,
                    lines: parts[0].to_string()
                });
            }
        }

        //println!("program_lines: {:?}", program_lines);

        let mut results: Vec<usize> = Vec::new();

        //let options: i32 = 2;
        let chars = &['.', '#'];


        for program_line in program_lines {
            let mut line_result = 0; 
            let to_check = program_line.instructions.clone();

            // get all ?
            //let how_many_unkown = program_line.lines.chars().filter(|&x| x == '?').count();
            //println!("how_many_unkown: {}", how_many_unkown);
            //println!("how many options: {}", options.pow(how_many_unkown as u32));

            let length = program_line.lines.chars().filter(|&c| c == '?').count();

            for i in 0..(1 << length) {
                let mut variation = String::new();
                let mut j = 0;
        
                for c in program_line.lines.chars() {
                    if c == '?' {
                        let bit = (i >> j) & 1;
                        variation.push(chars[bit as usize]);
                        j += 1;
                    } else {
                        variation.push(c);
                    }
                }
        
                //println!("{}", variation);

                let to_control = get_group_by_input(variation);

                if to_control == to_check {
                    line_result += 1;
                }

            }

            results.push(line_result);
        
        }

        println!("results: {:?}", results);

        println!("sum: {}", results.iter().sum::<usize>());

        
    }
}

fn get_group_by_input(variation: String) -> Vec<usize>{
    let mut result_vec = Vec::new();
    let mut count = 0;

    for c in variation.chars() {
        match c {
            '.' => {
                if count > 0 {
                    result_vec.push(count);
                }
                count = 0;
            }
            '#' => count += 1,
            _ => {}
        }
    }
            
    if count > 0 {
        result_vec.push(count);
    }
    return result_vec;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}