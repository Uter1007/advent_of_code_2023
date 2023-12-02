use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut endresult:i32 = 0;

    let word_to_digit: Vec<(&str, &str)> = vec![
        // special cases
        ("twone", "21"),
        ("eighthree", "83"),
        ("eightwo", "82"),
        ("oneight", "18"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("nineight", "98"),
        ("sevenine","79"),
        //basic cases
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {

                let mut modified_input = ip.to_string();

                for (word, digit) in word_to_digit.iter() {
                    modified_input = modified_input.replace(word, digit);
                }

                let digits: String = modified_input.chars().filter(|c| c.is_digit(10)).collect();
                if let (Some(first_digit), Some(last_digit)) = (digits.chars().next(), digits.chars().last()) {
                   
                    let result: String = first_digit.to_string() +""+ &last_digit.to_string();

                    println!("IP: {} -> {} -> {}", ip, modified_input, result);

                    let resutl_int: i32 = result.parse().unwrap();
                    endresult = endresult + resutl_int;

                }
            }
        }
    }

    println!("Endresult: {}", endresult);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}