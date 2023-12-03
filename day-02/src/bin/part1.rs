use std::fs;
use regex::Regex;

fn main() {
    let contents1 = fs::read_to_string("src/input1.txt").expect("Should have been able to read the file");

    let mut games: Vec<Game> = Vec::new();

    for line in contents1.lines() {
        games.push(get_game(line));
    }

    let possible_games = get_possible_games(games);

    let mut id_sum: u32 = 0;
    
    let do_print = true;

    if do_print {
        for game in possible_games {
            println!("Game: {}", game.id);
            id_sum = id_sum + game.id;
            for round in game.rounds {
                println!("Blue: {}", round.blue);
                println!("Green: {}", round.green);
                println!("Red: {}", round.red);
                println!("");
            }
        }
    }

    println!("{}", id_sum);
    
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

fn get_possible_games(games: Vec<Game>) -> Vec<Game> {
    let mut possible_games: Vec<Game> = Vec::new();

    for game in games {
        let mut possible = true;
        let game1 = game.clone();
        for round in game.rounds {
            if round.blue > 14 || round.green > 13 || round.red > 12 {
                possible = false;
            }
        }
        if possible {
            possible_games.push(game1);
        }
    }
    possible_games
}
