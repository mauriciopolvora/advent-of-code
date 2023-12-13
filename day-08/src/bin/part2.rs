use regex::Regex;
use std::{fs, collections::HashMap};
fn main() {
    let contents =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let instructions = get_instructions(&contents.lines().next().unwrap_or(""));
    let map = process_map(&contents);
    println!("Map: {:?}", map);
    println!("Instructions: {:?}", instructions);

    let mut values = Vec::new();
    
    for (key, _) in &map {
        if key.ends_with('A') {
            values.push(key.clone());
        }
    }

    println!("Values: {:?}", values);

    let mut steps = 0;
    let mut current: Vec<String> = values.clone();

    let mut keep = true;
    while keep {
        let (new_current, new_steps) = process_instructions(&map, &instructions, current, steps);
        current = new_current;
        steps = new_steps;
        if current.iter().all(|node| node.contains("Z")) {
            println!("Found ZZZ in {} steps", steps);
            keep = false;
        }
    }



}

fn get_instructions(line: &str) -> Vec<char> {
    let mut instructions: Vec<char> = Vec::new();

    for char in line.chars() {
        instructions.push(char);
    }

    instructions
}

fn process_instructions(map: &HashMap<String, (String, String)>, instructions: &[char], starting: Vec<String>, steps: u32) -> (Vec<String>, u32) {
    let mut current = starting;
    for &instruction in instructions {
        current = current.into_iter().filter_map(|node| {
            match instruction {
                'R' => {
                    let (_value1, value2) = map.get(&node).unwrap();
                    Some(value2.to_string())
                }
                'L' => {
                    let (value1, _value2) = map.get(&node).unwrap();
                    Some(value1.to_string())
                }
                _ => {
                    None
                }
            }
        }).collect();

        println!("Current: {:?}", current);
    }
    (current, steps + instructions.len() as u32)
}



fn process_map(lines: &str) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let re = Regex::new(r"(\w+)\s*=\s*\((\w+),\s*(\w+)\)").unwrap();

    for line in lines.lines().skip(2) {
        if let Some(caps) = re.captures(line) {
            let key = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let value1 = caps.get(2).map_or("", |m| m.as_str()).to_string();
            let value2 = caps.get(3).map_or("", |m| m.as_str()).to_string();
            map.insert(key, (value1, value2));
        }
    }

    map
}
