use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let program = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<String>();

    let dos = program.match_indices("do()");
    let donts = program.match_indices("don't()");

    let mut dos_and_donts = dos.chain(donts)
        .collect::<Vec<_>>();
    dos_and_donts.sort_by(|a, b| a.0.cmp(&b.0));
    let mut commands = dos_and_donts.iter().map(|(_, str)| str);

    let mut simplified_program = program.clone();
    while let Some(command) = commands.next() {
        if *command != "don't()" {
            continue;
        }
        let Some((first, last)) = split_at_str(&simplified_program, command) else {
            continue;
        };
        while let Some(command) = commands.next() {
            if *command != "do()" {
                continue;
            }
            if let Some((_first, last)) = split_at_str(last, command) {
                simplified_program = first.to_string() + last;
                break;
            }
        }
    }

    let muls = find_all_between(&simplified_program, "mul(", ")");
    let sum: u32 = muls.iter()
        .map(|str| mul_str(str))
        .sum();
    println!("{sum}");
}

fn find_all_between<'a>(str: &'a str, head: &'a str, tail: &'a str) -> Vec<&'a str> {
    let mut matches = vec![];
    for (index, _) in str.match_indices(head) {
        let (_, slice) = str.split_at(index + head.len());
        if let Some((found, _)) = split_at_str(slice, tail) {
            matches.push(found);
        };
    }
    matches
}

fn split_at_str<'a>(str: &'a str, pattern: &'a str) -> Option<(&'a str, &'a str)> {
    let Some(found) = str.find(pattern) else {
        return None;
    };
    let (first, last) = str.split_at(found);
    Some((first, last))
}

fn mul_str(str: &str) -> u32 {
    let Some((first, last)) = str.split_once(',') else {
        return 0;
    };
    let Ok(first) = first.parse::<u32>() else {
        return 0;
    };
    let Ok(last) = last.parse::<u32>() else {
        return 0;
    };
    first * last
}
