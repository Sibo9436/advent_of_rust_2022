// Advent of code 2022

use std::ops::RangeInclusive;

fn line_to_range(line: &str) -> RangeInclusive<i32> {
    let lrange = line.trim().split_once('-').unwrap();
    lrange.0.parse::<i32>().expect("NOOO")..=lrange.1.parse::<i32>().expect("NOOOO")
}

fn parse_input(input: &str) -> Vec<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    input
        .trim()
        .split('\n')
        .map(|line| line.split_once(',').unwrap())
        .map(|(left, right)| (line_to_range(left), line_to_range(right)))
        .collect()
}

fn first_part(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> u64 {
    let mut count = 0;
    for (left, right) in input {
        if (*left.start() - *right.start()) * (*left.end() - *right.end()) <= 0 {
            count += 1;
        }
    }
    count
}

fn second_part(input: &[(RangeInclusive<i32>, RangeInclusive<i32>)]) -> u64 {
    let mut count = 0;
    //I wonder if there's a more elegant way
    for (left, right) in input {
        let (smaller, bigger) = if left.start() <= right.start() {
            (left, right)
        } else {
            (right, left)
        };
        if smaller.end() >= bigger.start() {
            count += 1;
        }
    }
    count
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    let input_result = first_part(&input);
    println!("{:?}", test_result);
    println!("{:?}", input_result);
    let test_result = second_part(&test);
    println!("{:?}", test_result);
    let input_result = second_part(&input);
    println!("{:?}", input_result);
    println!("Hello, Rust");
}
