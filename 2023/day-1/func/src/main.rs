use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const SPELLED_NUMBERS: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let file = File::open("ex2")
        .unwrap_or_else(|e| panic!("{e}"));

    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let found = SPELLED_NUMBERS.iter()
                .enumerate()
                .filter_map(move |(num_index, num)| 
                if let Some(str_index) = line.find(*num) { 
                    Some((num_index, str_index.clone(), line.clone())) 
                } else {
                    None 
                })
                .map(|(num_index, str_index, line)| {
                    let mut line = line.clone();
                    line.insert_str(str_index, &num_index.to_string());
                    line
                }).collect::<String>();
            found
        })
    .collect::<Vec<String>>();

    for line in lines.iter() {
        println!("{line}");
    }

    let nums: u32 = lines.iter()
        .map(|line| line.chars()
            .filter_map(|char| char.to_digit(10))
            .collect::<Vec<u32>>()
        )
        .map(|nums| {
            let first = nums.first().unwrap_or(&0);
            let last = nums.last().unwrap_or(&0);
            first * 10 + last
        })
    .sum();

    println!("{nums}");
}
