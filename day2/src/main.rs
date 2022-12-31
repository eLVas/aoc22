use std::fs;
use std::path::PathBuf;

use enum_iterator::{all, Sequence};

use crate::GameResult::*;
use crate::Sign::*;

#[derive(Debug, PartialEq, Eq, Sequence)]
enum Sign {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq, Eq)]
enum GameResult {
    Defeat = 0,
    Draw = 3,
    Win = 6,
}

struct Rule {
    pub beats: Sign,
}

fn char_to_sign(ch: char) -> Sign {
    match ch {
        'A' | 'X' => Rock,
        'B' | 'Y' => Paper,
        'C' | 'Z' => Scissors,
        _ => panic!("Invalid input: {:?}", ch),
    }
}

fn char_to_result(ch: char) -> GameResult {
    match ch {
        'X' => Defeat,
        'Y' => Draw,
        'Z' => Win,
        _ => panic!("Invalid input: {:?}", ch),
    }
}

fn rock_paper_scissors_rule_lookup(s: &Sign) -> Rule {
    match s {
        Rock => Rule { beats: Scissors },
        Paper => Rule { beats: Rock },
        Scissors => Rule { beats: Paper },
    }
}

fn rock_paper_scissors(player: &Sign, opponent: &Sign) -> GameResult {
    if player == opponent {
        return Draw;
    }

    let rule = rock_paper_scissors_rule_lookup(player);

    if rule.beats == *opponent {
        Win
    } else {
        Defeat
    }
}

fn main() {
    let file_path = PathBuf::from("day2/resources/input.txt");
    let data_raw = fs::read_to_string(file_path).unwrap();

    let total: u32 = data_raw
        .lines()
        .map(|line| {
            let arr: Vec<char> = line.split(" ").map(|s| s.chars().nth(0).unwrap()).collect();
            assert_eq!(arr.len(), 2, "Invalid input {}", line);

            let player = char_to_sign(arr[1]);
            let opponent = char_to_sign(arr[0]);

            let result = rock_paper_scissors(&player, &opponent);

            player as u32 + result as u32
        })
        .sum();

    println!("Game total(Answer #1): {}", total);

    let total: u32 = data_raw
        .lines()
        .map(|line| {
            let arr: Vec<char> = line.split(" ").map(|s| s.chars().nth(0).unwrap()).collect();
            assert_eq!(arr.len(), 2, "Invalid input {}", line);

            let opponent = char_to_sign(arr[0]);
            let result = char_to_result(arr[1]);

            let player = (|| {
                for x in all::<Sign>() {
                    if rock_paper_scissors(&x, &opponent) == result {
                        return x;
                    }
                }

                panic!("Should not end up here")
            })();

            player as u32 + result as u32
        })
        .sum();

    println!("Game total(Answer #2): {}", total);
}
