use std::{fs, env::current_exe, ptr::read};
use indexmap::IndexMap;

#[derive(Debug)]
struct BoxValue {
    label: String,
    value: i64,
    hash: i64,
    del_operation: bool
}

fn main() {
    let file_path = "input2.txt";

    let mut inputstr = read_txt_file(file_path);

    let grid = read_comma_separated(inputstr);

    // println!("Grid: {:?}", grid);

    // Part1
    //println!("Test grid: {:?}", hash_algo(grid));

    let mut boxes: [IndexMap<String, i64>; 256] = (0..256).map(|_| IndexMap::new()).collect::<Vec<_>>().try_into().unwrap();

    let mut boxes2 = hash_map_algo(grid);

    for i in boxes2 {
        // println!("Box: {:?}", i);

        if i.del_operation {
            // println!("Remove: {}", i.label);
            boxes[i.hash as usize].shift_remove(&i.label);
        } else {

            check_and_modify_or_insert(&mut boxes[i.hash as usize], &i.label.to_string(), i.value as i64);
        }

        // println!("Boxes => : {:?}", boxes);
    }

    // println!("Boxes: {:?}", boxes);

    let mut sum: i64 = 0;

    for (index, box_version) in boxes.iter().enumerate() {
        for  (pos, (label, value)) in box_version.iter().enumerate() {
            

            let val: i64 = ((index + 1) as i64 ) * (pos+1) as i64 * value;
            println!("{}: {} {} {}",   pos, label, value, val);

            sum += val;
        }
    }

    println!("Sum: {}", sum);

}

fn check_and_modify_or_insert(map: &mut IndexMap<String, i64>, key: &str, new_value: i64) {
    if let Some(existing_value) = map.get_mut(key) {
        // Der Schlüssel existiert bereits, aktualisiere den Wert
        *existing_value = new_value;
    } else {
        // Der Schlüssel existiert nicht, füge einen neuen Eintrag hinzu
        map.insert(key.to_string(), new_value);
    }
}

fn read_comma_separated(input: String) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    
    let mut row: Vec<char> = Vec::new();
    
    for i in input.chars() {
        if i == ',' {
            grid.push(row);
            row = Vec::new();
        } else {
            row.push(i);
        }
    }

    if row.len() > 0 {
        grid.push(row);
    }

    return grid;
}

fn hash_map_algo(input: Vec<Vec<char>>) -> Vec<BoxValue> {
    
    let mut boxes: Vec<BoxValue> = Vec::new();
    for i in input {

        let whole_string: String = i.iter().collect(); 
        let splitted_string: Vec<&str> = whole_string.split(['=', '-'].as_ref()).collect();

        let number: i64 = match splitted_string[1].parse::<i64>() {
            Ok(parsed_number) => parsed_number,
            Err(_) => 0,
        };

        let box_value = BoxValue {
            label: splitted_string[0].to_string(),
            value: number,
            hash: get_hash_value(splitted_string[0].chars().collect()),
            del_operation: whole_string.contains("-")
        };

        boxes.push(box_value);

    }

    return boxes;
}

fn read_txt_file(file_path: &str) -> String {
    let input_str = fs::read_to_string(file_path)
        .expect("Fehler beim Lesen der Datei");
    return input_str;
}

fn hash_algo(input: Vec<Vec<char>>) -> i64{
    
    let mut sum_hash = 0;

    for i in input  {
        let current_value = get_hash_value(i);

        println!("Hash: {}", current_value);
        sum_hash += current_value;
    }

    println!("Sum Hash: {}", sum_hash);
    return sum_hash;
}

fn get_hash_value(i: Vec<char>) -> i64 {
    let mut current_value: i64 = 0;
    for j in i {
        let ascii_value = get_ascii_for_char(j);
        current_value = current_value + ascii_value;
        current_value = current_value * 17;
        current_value = current_value % 256;
    }
    current_value
}

fn get_ascii_for_char(input: char) -> i64 {
    let ascii_value = input as i64;
    return ascii_value;
}