use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("src/input1.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();
    let time: Vec<u32> = lines[0].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distance: Vec<u32> = lines[1].split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

    let mut map: HashMap<u32, u32> = HashMap::new();

    for (t, d) in time.iter().zip(distance.iter()) {
        map.insert(*t, *d);
    }

    let possible_wins = process_map(map);

    println!("Possible wins: {:?}", possible_wins);

    let mut total_wins: Vec<u32> = Vec::new();

    for (_time, wins) in possible_wins.iter() {

        let mut win = 0;

        for _wins in wins.iter() {
            win = win + 1;
        }  
        total_wins.push(win);  
    }

    let total_wins_product: u32 = total_wins.iter().fold(1, |product, &value| product * value);

    println!("Total wins product: {:?}", total_wins_product);

}

fn get_distance(time: u32, speed: u32) -> u32 {
    time * speed
}

fn process_map(map: HashMap<u32, u32>) -> HashMap<u32, Vec<u32>> {

    let mut possible_wins:HashMap<u32, Vec<u32>> = HashMap::new();

    for (time, distance) in map.iter() {

        let total_time = *time;


        for speed in 1..total_time {
            let try_distance = get_distance(total_time - speed, speed);
            if try_distance > *distance {
                possible_wins.entry(total_time).or_insert(Vec::new()).push(try_distance);
            }
        }
   
    }

    possible_wins
}