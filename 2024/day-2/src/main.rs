use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let file = File::open("input.txt").unwrap();

    let reports = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| { line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect_vec()
        })
    .filter(|report| is_safe_dampened(report))
    .collect::<Vec<_>>();

    println!("{}", reports.len());
}

fn is_safe(report: &Vec<i32>) -> bool {
    let iter = report.iter();
    if !iter.clone().is_sorted() && !iter.rev().is_sorted() {
        return false;
    }

    report
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| (a - b).abs())
        .all(|x| (1..=3).contains(&x))
}

fn is_safe_dampened(report: &Vec<i32>) -> bool {
    if is_safe(report) == true {
        return true;
    }

    for index in 0..report.len() {
        let mut clone = report.clone();
        clone.remove(index);
        if is_safe(&clone) {
            return true;
        }
    }
    
    return false;
}
