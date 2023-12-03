use std::fs;
use regex::Regex;

fn main() {
    let contents1 = fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let mut games: Vec<Game> = Vec::new();

    for line in contents1.lines() {
        games.push(get_game(line));
    }

    let mut sum: u32 = 0;

    for game in games {
        sum += get_minimum_amount_of_color(game);
    }

    println!("{}", sum);
    
}

#[derive(Clone)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Clone)]
struct Round {
    blue: u32,
    green: u32,
    red: u32,
}


fn get_game(input: &str) -> Game {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let re_id = Regex::new(r"(Game) (\d+)").unwrap();

    let rounds: Vec<&str> = input.split(';').collect();

    let mut game = Game {
        id: 0,
        rounds : Vec::new(),
    };

    for cap in re_id.captures_iter(input) {
        let id: u32 = cap[2].parse().unwrap();
        game.id = id;
    }


    for round in rounds {

        let mut round_s = Round {
            blue: 0,
            green: 0,
            red: 0,
        };

        for cap in re.captures_iter(round) {
            let count: u32 = cap[1].parse().unwrap();
            let color = &cap[2];

            match color {
                "blue" => round_s.blue += count,
                "green" => round_s.green += count,
                "red" => round_s.red += count,
                _ => (),
            }
        }

        game.rounds.push(round_s);

    }
    game
}

fn get_minimum_amount_of_color(game: Game) -> u32 {
    let mut min_blue: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_red: u32 = 0;

    for round in game.rounds {
        if round.blue > min_blue {
            min_blue = round.blue;
        }
        if round.green > min_green {
            min_green = round.green;
        }
        if round.red > min_red {
            min_red = round.red;
        }
    }

    //println!("{} {} {}", min_blue, min_green, min_red);

    min_blue * min_green * min_red
}
