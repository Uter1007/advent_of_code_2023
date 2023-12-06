use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug, Clone)]

struct InputData {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<Vec<u64>>,
    soil_to_fertilizer_map: Vec<Vec<u64>>,
    fertilizer_to_water_map: Vec<Vec<u64>>,
    water_to_light_map: Vec<Vec<u64>>,
    light_to_temperature_map: Vec<Vec<u64>>,
    temperature_to_humidity_map: Vec<Vec<u64>>,
    humidity_to_location_map: Vec<Vec<u64>>,
}

fn main() {
    let file_path = "input.txt";

    if let Ok(file) = File::open(file_path) {
        let reader = io::BufReader::new(file);

        let mut result: Vec<i64> = Vec::new();

        fn parse_line(line: &str) -> Vec<u64> {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        }

        let mut input_data = InputData {
            seeds: Vec::new(),
            seed_to_soil_map: Vec::new(),
            soil_to_fertilizer_map: Vec::new(),
            fertilizer_to_water_map: Vec::new(),
            water_to_light_map: Vec::new(),
            light_to_temperature_map: Vec::new(),
            temperature_to_humidity_map: Vec::new(),
            humidity_to_location_map: Vec::new(),
        };

        let mut current_header: Option<String> = None;

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.ends_with(':') {
                    current_header = Some(line.trim_end_matches(':').to_string());
                    continue;
                }

                let values = parse_line(&line);

                if values.len() == 0 {
                    continue;
                }

                if current_header.clone().unwrap_or_default().len() == 0 {
                    input_data.seeds.extend(values);
                    continue;
                }

                match current_header.as_deref() {
                    Some("seed-to-soil map") => input_data.seed_to_soil_map.push(values),
                    Some("soil-to-fertilizer map") => input_data.soil_to_fertilizer_map.push(values),
                    Some("fertilizer-to-water map") => input_data.fertilizer_to_water_map.push(values),
                    Some("water-to-light map") => input_data.water_to_light_map.push(values),
                    Some("light-to-temperature map") => input_data.light_to_temperature_map.push(values),
                    Some("temperature-to-humidity map") => input_data.temperature_to_humidity_map.push(values),
                    Some("humidity-to-location map") => input_data.humidity_to_location_map.push(values),
                    _ => (),
                }
            }
        }

        // println!("Seeds: {:?}", input_data.seeds);
        // println!("Seed to soil map: {:?}", input_data.seed_to_soil_map);
        // println!("Soil to fertilizer map: {:?}", input_data.soil_to_fertilizer_map);
        // println!("Fertilizer to water map: {:?}", input_data.fertilizer_to_water_map);
        // println!("Water to light map: {:?}", input_data.water_to_light_map);
        // println!("Light to temperature map: {:?}", input_data.light_to_temperature_map);
        // println!("Temperature to humidity map: {:?}", input_data.temperature_to_humidity_map);
        // println!("Humidity to location map: {:?}", input_data.humidity_to_location_map);

        for seed in &input_data.seeds {
            println!("Seed: {}", seed);
            let mut ret_soil = *seed as i64;

            for soil in input_data.seed_to_soil_map.clone() {
                let res = get_mapping_value(*seed,soil);
                if res > 0 {
                    //println!("Soil: {}", res);
                    ret_soil = res;
                }
            }

            println!("Soil: {}", ret_soil);
            let mut ret_fertilizer: i64 = ret_soil;

            for fertilizer in input_data.soil_to_fertilizer_map.clone() {
                let res = get_mapping_value(ret_soil as u64,fertilizer);
                if res > 0 {
                    // println!("Fertilizer: {}", res);
                    ret_fertilizer = res;
                }
            }

            println!("Fertilizer: {}", ret_fertilizer);

            let mut ret_water: i64 = ret_fertilizer;

            for water in input_data.fertilizer_to_water_map.clone() {
                let res = get_mapping_value(ret_fertilizer as u64,water);
                if res > 0 {
                    // println!("Water: {}", res);
                    ret_water = res;
                }
            }

            println!("Water: {}", ret_water);

            let mut ret_light: i64 = ret_water;

            for light in input_data.water_to_light_map.clone() {
                let res = get_mapping_value(ret_water as u64,light);
                if res > 0 {
                    // println!("Light: {}", res);
                    ret_light = res;
                }
            }

            println!("Light: {}", ret_light);


            let mut ret_temperature: i64 = ret_light;

            for temperature in input_data.light_to_temperature_map.clone() {
                let res = get_mapping_value(ret_light as u64,temperature);
                if res > 0 {
                    // println!("Temperature: {}", res);
                    ret_temperature = res;
                }
            }

            println!("Temperature: {}", ret_temperature);

            let mut ret_humidity: i64 = ret_temperature;

            for humidity in input_data.temperature_to_humidity_map.clone() {
                let res = get_mapping_value(ret_temperature as u64,humidity);
                if res > 0 {
                    // println!("Humidity: {}", res);
                    ret_humidity = res;
                }
            }

            println!("Humidity: {}", ret_humidity);

            let mut ret_location: i64 = ret_humidity;

            for location in input_data.humidity_to_location_map.clone() {
                let res = get_mapping_value(ret_humidity as u64,location);
                if res > 0 {
                    // println!("Location: {}", res);
                    ret_location = res;
                }
            }

            println!("Location: {}", ret_location);

            result.push(ret_location);
        }

        let smallest_location = result.iter().cloned().min();

        println!("Smallest location: {:?}", smallest_location);
    
    } else {
        println!("Fehler beim Öffnen der Datei: {}", file_path);
    }
}

fn get_mapping_value(input: u64, mapping: Vec<u64>) -> i64 {
    let destination = mapping[0];
    let source = mapping[1];
    let length = mapping[2];
    if input >= source && input <= source + length {
        // I don't know how to cast to i64
        let diff2 = destination as i64 - source as i64;
        return input as i64 + diff2;
      } else {
        return 0;
      }
}
