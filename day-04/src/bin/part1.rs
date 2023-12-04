use std::fs;

fn main() {
    let contents1 =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let mut sum: u32 = 0;

    for line in contents1.lines() {
        let winning_numbers: Vec<u32> = find_overlap(get_winning_numbers(line), get_playing_numbers(line));
        if winning_numbers.len() > 0 {
            sum = sum + (2u32.pow(winning_numbers.len() as u32 - 1));
        }
        println!("Winning numbers: {:?}", winning_numbers);
        println!("");
    }

    println!("The sum is {}", sum);

}

fn get_winning_numbers(line: &str) -> Vec<u32> {

    let rounds: Vec<&str> = line.split('|').collect();
    let winning_numbers: Vec<u32> = rounds[0]
        .split_whitespace()
        .skip(2)
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", winning_numbers);

    winning_numbers
}

fn get_playing_numbers(line: &str) -> Vec<u32> {
    let rounds: Vec<&str> = line.split('|').collect();
    let playing_numbers: Vec<u32> = rounds[1]
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", playing_numbers);

    playing_numbers
}

fn find_overlap(winning_numbers: Vec<u32>, playing_numbers: Vec<u32>) -> Vec<u32> {
    let mut overlap: Vec<u32> = Vec::new();

    for number in winning_numbers {
        if playing_numbers.contains(&number) {
            overlap.push(number);
        }
    }

    overlap
}