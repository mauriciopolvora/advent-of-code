use std::fs;

fn main() {
    let contents1 =
        fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    println!("The sum is {}", part_2(contents1));

}

#[derive(Debug)]
struct Card {
    _id: u32,
    left: Vec<i32>,
    right: Vec<i32>,
}

//Creating vector of games, then creating vector of matches(wins, per game)
pub fn part_2(input: String) -> u32 {
    let games = input
        .lines()
        .map(|x| {
            let (id, lr) = x.split_once(':').unwrap();
            let id = id.chars().last().unwrap().to_digit(10).unwrap();

            let (left, right) = lr.trim().split_once('|').unwrap();

            let left = left
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            let right = right
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            Card {
                _id: id,
                left: left,
                right: right,
            }
        })
        .collect::<Vec<_>>();

    //Calculating How many wins
    let matches = games
        .iter()
        .map(|card| {
            let amm = card
                .left
                .iter()
                .map(|x| {
                    if let Some(_) = card.right.iter().find(|y| *y == x) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<u32>();
            amm
        })
        .collect::<Vec<_>>();

    // THE RECURSIVE PART
    (0..matches.len()).map(|i| part_2_rec(i, 0, &matches)).sum()
}

//Here 
fn part_2_rec(i: usize, accum: u32, matches: &Vec<u32>) -> u32 {
    if let Some(_) = matches.get(i) {
        let amm = matches[i];

        (1..=amm as usize)
            .map(|j| part_2_rec(i + j, accum + amm, matches))
            .fold(1, |total, x| total + x)
    } else {
        accum
    }
}