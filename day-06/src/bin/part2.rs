use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("src/input1.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.lines().map(|line| line.replace(" ", "")).collect();
    let time: Vec<u64> = lines[0].split(":").skip(1).map(|s| s.parse().unwrap()).collect();
    let distance: Vec<u64> = lines[1].split(":").skip(1).map(|s| s.parse().unwrap()).collect();

    let mut map: HashMap<u64, u64> = HashMap::new();

    for (t, d) in time.iter().zip(distance.iter()) {
        map.insert(*t, *d);
    }

    let possible_wins = process_map(map);

    //println!("Possible wins: {:?}", possible_wins);

    let mut total_wins: Vec<u64> = Vec::new();

    for (_time, wins) in possible_wins.iter() {

        let mut win = 0;

        for _wins in wins.iter() {
            win = win + 1;
        }  
        total_wins.push(win);  
    }

    let total_wins_product: u64 = total_wins.iter().fold(1, |product, &value| product * value);

    println!("Total wins product: {:?}", total_wins_product);

}

fn get_distance(time: u64, speed: u64) -> u64 {
    time * speed
}

fn process_map(map: HashMap<u64, u64>) -> HashMap<u64, Vec<u64>> {

    let mut possible_wins:HashMap<u64, Vec<u64>> = HashMap::new();

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