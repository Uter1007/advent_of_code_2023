use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    if let Err(err) = parse_input_from_file("input.txt") {
        eprintln!("Error: {}", err);
    }
}

fn parse_input_from_file(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let numbers: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        
        rows.push(numbers);
    }

    //println!("{:?}", rows);

    let mut next_values:Vec<i64> = Vec::new();

    for res in rows.iter() {

        //println!("res: {:?}", res);
        let mut check_matrix:Vec<Vec<i64>> = Vec::new();
        check_matrix.push(res.clone());
        check_matrix.push(calculate_differences(res));

        while !check_differences_are_all_zeroes(check_matrix.last().unwrap()) {
            check_matrix.push(calculate_differences(check_matrix.last().unwrap()));
        }

        //println!("check_matrix: {:?}", check_matrix);

        check_matrix.reverse();

        let mut last_value = 0;

        for check_row in check_matrix.iter() {
            last_value = last_value + check_row.last().unwrap();
        }

        //println!("last_value: {}", last_value);

        next_values.push(last_value);

    }

    println!("next_values: {:?}", next_values);

    let mut end_res = 0;

    for res in next_values.iter() {
        end_res = end_res + res;
    }

    println!("end_res: {}", end_res);


    let mut prev_values:Vec<i64> = Vec::new();

    for res in rows.iter() {

        //println!("res: {:?}", res);
        let mut check_matrix:Vec<Vec<i64>> = Vec::new();
        check_matrix.push(res.clone());
        check_matrix.push(calculate_differences(res));

        while !check_differences_are_all_zeroes(check_matrix.last().unwrap()) {
            check_matrix.push(calculate_differences(check_matrix.last().unwrap()));
        }

        //println!("check_matrix: {:?}", check_matrix);

        check_matrix.reverse();

        let mut first_value = 0;

        for check_row in check_matrix.iter() {
            first_value = check_row.first().unwrap() - first_value;
        }

        //println!("last_value: {}", last_value);

        prev_values.push(first_value);

    }

    println!("prev_values: {:?}", prev_values);

    let mut end_res2 = 0;

    for res in prev_values.iter() {
        end_res2 = end_res2 + res;
    }

    println!("end_res2: {}", end_res2);


    Ok(())
}

fn check_differences_are_all_zeroes(differences: &Vec<i64>) -> bool {
    for difference in differences {
        if *difference != 0 {
            return false;
        }
    }

    true
}

fn calculate_differences(numbers: &Vec<i64>) -> Vec<i64> {
    let mut differences = Vec::new();

    for i in 1..numbers.len() {
        differences.push(numbers[i] - numbers[i - 1]);
    }

    differences
}