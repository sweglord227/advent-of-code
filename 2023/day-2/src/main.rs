use std::fs::File;
use std::io::{BufReader, BufRead};
use strum::IntoEnumIterator;
use strum::{EnumIter, Display};
use itertools::Itertools;

#[derive(Default, Clone, Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(EnumIter, Display)]
enum Colors {
    Red,
    Green,
    Blue,
}

const BAG_CONTAINS: Round = Round {
    red: 12,
    green: 13,
    blue: 5
};

fn main() {
    let file = File::open("ex")
        .unwrap_or_else(|_| panic!("failed to open file"));
    let buf_reader = BufReader::new(file);
    let games = deserialize_file(buf_reader);
    for game in games {
        for round in game {
            println!("{round:?}");
        }
        println!("");
    }
}

fn deserialize_file(buf_reader: BufReader<File>) -> Vec<Vec<Round>> {
    let mut rounds: Vec<Vec<Round>> = vec![];
    for line in buf_reader.lines() {
        let line = line
            .unwrap_or_else(|_| panic!("failed to read lines"));
        rounds.push(deserialize_line(line));
    }
    return rounds;
}

fn deserialize_line(line: String) -> Vec<Round> {
    let mut game: Vec<Round> = vec![];
    let mut round = Round::default();

    let line = line
        .chars()
        .filter(|&char| char.is_alphanumeric())
        .collect::<String>();
    let words: Vec<(&str, &str)> = line
        .split_whitespace()
        .filter(|&word| word != "Game")
        .tuples::<(_, _)>()
        .collect();
    for (value, key) in words.iter() {
        println!("{}, {}", key, value);
    }
    // while let Some(word) = words.next() {
        // if word == "Game" {
        //     _ = words.next();
        //     continue;
        // }
        // let num: u8 = word.parse()
        //     .unwrap_or_else(|e| panic!("parse error {e}"));
        // let Some(word) = words.next() else { break };
        //
        // Colors::iter().for_each(|color| {
        //     if word.find(&color.to_string().to_lowercase()).is_some() {
        //         match color {
        //             Colors::Red => round.red = num,
        //             Colors::Blue => round.blue = num,
        //             Colors::Green => round.green = num,
        //         }
        //     }
        // });
        // for color in Colors::iter() {
        // }

    //     if word.chars().last().expect("string will have chars") == ';' {
    //         game.push(round);
    //         round = Round::default();
    //     }
    // }
    return game;
}


