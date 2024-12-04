use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let Ok(file) = File::open("input") else {
        panic!("file does not exist");
    };
    let buffer =  BufReader::new(file);
    let lines = replace_spelled_nums_in_buf(buffer);
    let digits = find_digits_in_buf(lines);
    let sum = add_calibration_values(&digits);
    println!("{}", sum);
}

fn replace_spelled_nums_in_buf(buffer: BufReader<File>) -> Vec<String> {
    let mut parsed_lines: Vec<String> = vec![];
    for line in buffer.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => panic!("{e}"),
        };
        parsed_lines.push(replace_spelled_nums_in_str(line));
    }
    return parsed_lines;
}

const SPELLED_NUMBERS: [&str;10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

#[derive(Debug, Clone)]
struct FoundDigit {
    str_index: usize,
    num_index: usize
} impl FoundDigit {
    fn new(str_index: usize, num_index: usize) -> Self {
        return Self { str_index, num_index }
    }
}

fn replace_spelled_nums_in_str(mut line: String) -> String {
    let mut maybe_first: Option<FoundDigit> = None;
    let mut maybe_last: Option<FoundDigit> = None;

    for num_index in 0..SPELLED_NUMBERS.len() {
        maybe_first = set_or_change_found_digit(
            maybe_first,
            line.find(SPELLED_NUMBERS[num_index]),
            num_index
        );
        maybe_last = set_or_change_found_digit(
            maybe_last,
            line.rfind(SPELLED_NUMBERS[num_index]),
            num_index
        );
    }

    if let Some(first) = maybe_first {
        line.insert_str(first.str_index, &(first.num_index).to_string())
    };
    if let Some(last) = maybe_last {
        let index = last.str_index + SPELLED_NUMBERS[last.num_index].len() + 1;
        line.insert_str(index, &(last.num_index).to_string())
    };

    return line;
}

fn set_or_change_found_digit(
    maybe_found: Option<FoundDigit>,
    str_index: Option<usize>,
    num_index: usize
) -> Option<FoundDigit> {
    let Some(str_index) = str_index else {
        return None;
    };
    let Some(found) = maybe_found else {
        return Some(FoundDigit::new(str_index, num_index));
    };

    let found_digit: FoundDigit;
    if found.str_index < str_index {
        found_digit = FoundDigit::new(str_index, num_index);
    } else {
        found_digit = found;
    }

    return Some(found_digit);
}


fn find_digits_in_buf(lines: Vec<String>) -> Vec<(u32, u32)>  {
    let mut digits: Vec<(u32, u32)> = vec![];
    for line in lines.iter() {
        digits.push(find_digits_in_str(&line));
    }
    return digits;
}

fn find_digits_in_str(line: &str) -> (u32, u32) {
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    for char in line.chars() {
        let int = char.to_digit(10);
        let Some(int) = int else {continue};
        if first == 0 {
            first = int;
        } else {
            last = int;
        }
    }
    return (first, last);
}

fn add_calibration_values(digits: &Vec<(u32, u32)>) -> u32 {
    let mut sum = 0;
    for (first, last) in digits.iter() {
        let add: u32;
        if *last != 0 {
            add = get_calibration_value(first, last);
        } else {
            add = get_calibration_value(first, first);
        }
        sum += add;
    }
    return sum;
}

fn get_calibration_value(first: &u32, last: &u32) -> u32 {
    return first * 10 + last;
}
