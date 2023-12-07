use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Run {
    time: u64,
    distance: u64,
}

fn parse_input(file_path: &str) -> Result<Vec<Run>, io::Error> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut runs = Vec::new();

    let mut lines = reader.lines().peekable();

    while let Some(Ok(time_line)) = lines.next() {
        let distance_line = lines.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "Incomplete pair of lines")
        })??;

        let time_values: Vec<u64> = parse_line_values(&time_line);
        let distance_values: Vec<u64> = parse_line_values(&distance_line);

        for (&time, &distance) in time_values.iter().zip(distance_values.iter()) {
            let run = Run { time, distance };
            runs.push(run);
        }
    }

    Ok(runs)
}

fn parse_line_values(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

fn calc_travel_distance(hold_time: u64, total_time: u64) -> u64 {

    const ACCERLERATION_MILLIMETERS_PER_MILLISECONDS: u64 = 1;

    let mut speed_time = 0;
    let mut total_distance = 0;

    if hold_time == 0 {
        return 0;
    }

    if hold_time >= total_time {
        return 0;
    }

    let difftime = total_time - hold_time;

    for _ in 0..hold_time {
        speed_time += ACCERLERATION_MILLIMETERS_PER_MILLISECONDS;
    }

    for _ in hold_time..total_time {
        total_distance += speed_time;
    }

    return total_distance

}

fn combine_values(values: Vec<u64>) -> u64 {
    let combined_string: String = values.iter().map(|&v| v.to_string()).collect();
    combined_string.parse().unwrap_or(0)
}

fn main() {
    let file_path = "input.txt";

    // let res0 = calc_travel_distance(0, 7);
    // let res1 = calc_travel_distance(1, 7);
    // let res2 = calc_travel_distance(2, 7);
    // let res3 = calc_travel_distance(3, 7);
    // let res4 = calc_travel_distance(4, 7);
    // let res5 = calc_travel_distance(5, 7);
    // let res6 = calc_travel_distance(6, 7);
    // let res7 = calc_travel_distance(7, 7);

    // println!("res0: {}", res0);
    // println!("res1: {}", res1);
    // println!("res2: {}", res2);
    // println!("res3: {}", res3);
    // println!("res4: {}", res4);
    // println!("res5: {}", res5);
    // println!("res6: {}", res6);
    // println!("res7: {}", res7);

    let mut res = 1;

    let mut total_time: Vec<u64> = Vec::new();
    let mut total_distance: Vec<u64> = Vec::new();

    match parse_input(file_path) {
        Ok(runs) => {
            for run in runs {
                println!("{:?}", run);
                total_time.push(run.time);
                total_distance.push(run.distance);
                let mut res_run = 0;
                for hold_time in 0..run.time {
                    let res = calc_travel_distance(hold_time, run.time);
                    if res > run.distance {
                        res_run += 1;
                    }
                }

                println!("res_run: {}", res_run);

                res *= res_run; 
            }

            let combine_time = combine_values(total_time);
            let combine_distance = combine_values(total_distance);

            println!("combine_time: {}", combine_time);
            println!("combine_distance: {}", combine_distance);

            let res2 = 1;
            let mut res_run2 = 0;
            for hold_time in 0..combine_time {
                let res2 = calc_travel_distance(hold_time, combine_time);
                if res2 > combine_distance {
                    res_run2 += 1;
                }
            }

            println!("res_run2: {}", res_run2);
        }

        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("res: {}", res);

}
