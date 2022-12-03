// Advent of code 2022
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
}

fn calc_value(ch: &char) -> u64 {
    if ch.is_ascii_uppercase() {
        *ch as u64 - 'A' as u64 + 27
    } else if ch.is_ascii_lowercase() {
        *ch as u64 - 'a' as u64 + 1
    } else {
        panic!("NONE");
    }
}

fn first_part(input: &[&str]) -> u64 {
    let splits: Vec<(&str, &str)> = input
        .iter()
        .map(|line| {
            let h = &line.len() / 2;
            (&line[0..h], &line[h..])
        })
        .collect();
    let mut total_value = 0;
    for (first, second) in splits {
        let mut found: HashSet<char> = HashSet::new();
        for ch in first.chars() {
            found.insert(ch);
        }
        for ch in second.chars() {
            if found.contains(&ch) {
                total_value += calc_value(&ch);
                break;
            }
        }
    }
    total_value
}

fn hash_rucksack(rucksack: &str) -> u64 {
    let mut hash: u64 = 0;

    for ch in rucksack.chars() {
        let value = calc_value(&ch);
        hash |= 1 << value;
    }
    hash
}

fn from_binary(b: u64) -> char {
    let mut value = 0;
    let mut o = 1;
    while o != b {
        value += 1;
        o <<= 1;
    }
    if value <= 26 {
        ((value - 1) as u8 + b'a') as char
    } else if value <= 52 {
        ((value - 27) as u8 + b'A') as char
    } else {
        println!("{b} {value}");
        panic!("COME ON");
    }
}

fn second_part(input: &Vec<&str>) -> u64 {
    if input.len() % 3 != 0 {
        panic!("Input size is not a multiple of three")
    };
    let hashes: Vec<u64> = input
        .iter()
        .map(|line| hash_rucksack(line.trim()))
        .collect();
    let mut i = 0;
    let mut total = 0;
    while i < hashes.len() {
        let first = hashes[i];
        let second = hashes[i + 1];
        let third = hashes[i + 2];
        let common = first & second & third;
        let common = from_binary(common);
        total += calc_value(&common);
        i += 3;
    }
    total
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    println!("1. test result: {test_result}");
    let input_result = first_part(&input);
    println!("1. input result: {input_result}");
    let test_result = second_part(&test);
    println!("2. test result: {test_result}");
    let input_result = second_part(&input);
    println!("2. input result: {input_result}");
    println!("Hello, Rust");
}
