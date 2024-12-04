use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|lines| lines.ok())
        .collect::<Vec<_>>();
    let mut list_a = vec![];
    let mut list_b = vec![];
    lines.iter()
        .map(|line| line
            .split_whitespace()
            .into_iter()
            .filter_map(|word| word.parse::<i32>().ok())
            .collect_tuple::<(_, _)>().expect("there should be 2 elements")
        )
        .for_each(|(a, b)| {
            list_a.push(a);
            list_b.push(b);
        });
    list_a.sort();
    list_b.sort();
    let mut similarity_mults: HashMap<i32, i32> = HashMap::new();
    list_b.iter().for_each(|num| {
        if let Some(mult) = similarity_mults.get_mut(num) {
            *mult += 1;
        } else {
            similarity_mults.insert(*num, 1);
        }
    });
    let result: i32 = list_a.iter()
        .map(|num| {
            if let Some(mult) = similarity_mults.get(num) {
                num * mult
            } else {
                0
            }
        })
        .sum();
    println!("{result}");
}
