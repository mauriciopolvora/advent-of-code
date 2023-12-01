use std::fs;
use aho_corasick::AhoCorasick;

fn main() {
    let contents1 = fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let mut sum = 0;

    for line in contents1.lines() {
        sum += get_numbers_from_string(line);

    }

    println!("{}", sum);
}

const PATTERNS: [&str; 19] = [
    "one", "two", "three", "four", "five", "six", "seven",
    "eight", "nine", "0", "1", "2", "3", "4", "5", "6",
    "7", "8", "9",
];


fn get_numbers_from_string(line: &str) -> u32{
    let ac = AhoCorasick::new(PATTERNS).unwrap();

    let mut it = ac.find_overlapping_iter(line);
    let first = from_matchables(
        PATTERNS[it
            .next()
            .expect("should be a number")
            .pattern()],
    );

    match it
        .last()
        .map(|mat| from_matchables(PATTERNS[mat.pattern()]))
    {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

fn from_matchables(input: &str) -> u32 {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => panic!("unexpected number!"),
    }
}