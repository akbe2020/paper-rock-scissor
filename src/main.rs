// Paper Rock Scissor
// Copyright (c) 2024 four4tReS
// MIT License

use std::io;
use rand::Rng;

#[derive(Debug)]
#[derive(PartialEq)]
enum Figure {
    Rock,
    Paper,
    Scissor
}

impl Figure {
    fn string_to_option_figure(some_string: String) -> Option<Figure> {
        match some_string.as_str() {
            "Rock" | "r" => Some(Figure::Rock),
            "Paper" | "p" => Some(Figure::Paper),
            "Scissor" | "s" => Some(Figure::Scissor),
            _ => None
        }
    }
}

#[derive(Debug)]
enum MatchResult {
    Win,
    Tie,
    Fail
}

fn generate_random_figure() -> Figure {
    let option_figure: Option<Figure>;

    match rand::thread_rng().gen_range(1..=3) {
        1 => option_figure = Some(Figure::Rock),
        2 => option_figure = Some(Figure::Paper),
        3 => option_figure = Some(Figure::Scissor),
        _ => option_figure = None // Never executes, but compiler forced me to create it
    }

    option_figure.unwrap()
}

// Returns match result for first player
fn get_match_result(first: Figure, second: Figure) -> MatchResult {
    if first == second {
        return MatchResult::Tie;
    }

    match first {
        Figure::Paper => {
            if second == Figure::Scissor {
                return MatchResult::Fail;
            }
        }
        Figure::Rock => {
            if second == Figure::Paper {
                return MatchResult::Fail;
            }
        }
        Figure::Scissor => {
            if second == Figure::Rock {
                return MatchResult::Fail;
            }
        }
    }

    MatchResult::Win
}

fn read_line() -> String {
    let mut some_string = String::new();

    io::stdin()
        .read_line(&mut some_string)
        .expect("Failed to read line");

    some_string.trim().to_string()
}



fn get_players_figure() -> Figure {
    println!("Input your Figure [Rock(r), Paper(p), Scissor(s)]");

    let input = read_line();

    match Figure::string_to_option_figure(input) {
        Some(f) => Some(f).unwrap(),
        None => {
            println!("Incorrect option");
            get_players_figure()
        }
    }
}

fn tick(players_score: &mut i32, bots_score: &mut i32) {
    let players_figure = get_players_figure();
    let bots_figure = generate_random_figure();
    
    println!("Bot selected figure: {:?}", bots_figure);

    let match_result = get_match_result(
        players_figure, bots_figure);

    match match_result {
        MatchResult::Win => *players_score += 1,
        MatchResult::Tie => {
            *players_score += 1;
            *bots_score += 1;
        }
        MatchResult::Fail => *bots_score += 1
    }

    println!("{:?}", match_result);
    println!("Score {}-{}\n", players_score, bots_score);
}

fn main() {
    println!("Welcome to Paper Rock Scissor");
    println!("Copyright (c) 2024 four4tReS");

    let mut players_score = 0;
    let mut bots_score = 0;

    loop {
        tick(&mut players_score, &mut bots_score);
    }
}
