use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut endresult:i32 = 0;

    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let digits: String = ip.chars().filter(|c| c.is_digit(10)).collect();
                if let (Some(first_digit), Some(last_digit)) = (digits.chars().next(), digits.chars().last()) {
                    println!("IP: {}", ip);
                    let result: String = first_digit.to_string() +""+ &last_digit.to_string();
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