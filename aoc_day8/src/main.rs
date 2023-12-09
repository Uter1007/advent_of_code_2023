use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ptr::null;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut steps_count = 0;
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut directions = String::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index == 1 {
            continue;
        }

        if index == 0 {
            // Erste Zeile enthält Richtungen
            directions = line.trim().to_string();
        } else {
            // Verarbeite die restlichen Zeilen
            let parts: Vec<&str> = line.split_whitespace().collect();
            let name = parts[0];
            let content_left = parts[2];
            let content_right = parts[3];

            
            let left = &content_left[1..content_left.len() - 1];
            let right = &content_right[0..content_right.len() - 1];

            let node = Node {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            };

            nodes.insert(name.to_string(), node);
        }
    }

    println!("Directions: {}", directions);
    println!("{:?}", nodes);

    let mut current_node = nodes.get("AAA").unwrap();
    
    while current_node.name != "ZZZ" {
        for direction in directions.chars() {
            steps_count += 1;
            match direction {
                'L' => {
                    current_node = nodes.get(&current_node.left).unwrap();
                },
                'R' => {
                    current_node = nodes.get(&current_node.right).unwrap();
                },
                _ => panic!("Invalid direction"),
            }
        }
    }
    
    println!("Steps: {}", steps_count);

    Ok(())
}
