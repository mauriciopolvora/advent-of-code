use regex::Regex;
use std::{fs, collections::HashMap};
fn main() {

    let contents =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let instructions = get_instructions(&contents.lines().next().unwrap_or(""));
    let map = process_map(&contents);

    let starting = "AAA".to_string();
    let steps = 0;
    let (current, steps) = process_instructions(&map, &instructions, starting, steps);

    println!("Current: {:?}", current);
    println!("Steps: {:?}", steps);


    println!("Map: {:?}", map);
    println!("Instructions: {:?}", instructions);
}

fn get_instructions(line: &str) -> Vec<char> {
    let mut instructions: Vec<char> = Vec::new();

    for char in line.chars() {
        instructions.push(char);
    }

    instructions
}

fn process_instructions(map: &HashMap<String, (String, String)>, instructions: &[char], starting: String, steps: u32) -> (String, u32) {
    let mut current = starting;
    for &instruction in instructions {
        match instruction {
            'R' => {
                let (_value1, value2) = map.get(&current).unwrap();
                current = value2.to_string();
            }
            'L' => {
                let (value1, _value2) = map.get(&current).unwrap();
                current = value1.to_string();
            }
            _ => {
                break;
            }
        }
        if current == "ZZZ" {
            println!("Found ZZ in {} steps", steps + instructions.len() as u32);
            break;
        }
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
