extern crate core;

use std::cmp::max;


// Advent of code 2022
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.trim().split('\n')
        .map(|line| {
            line.as_bytes().iter().map(|b| (b - b'0')).collect()
        }).collect()
}

const DIRECTIONS: [(i32, i32); 4] = [
    (1, 0),
    (0, -1),
    (-1, 0),
    (0, 1)
];
fn is_in_bounds(x:i32,y:i32, input : &Vec<Vec<u8>>)->bool{
    (y < input.len() as i32 && y >= 0) && (x < input[0].len() as i32 && x >= 0)
}
fn check_visibility( ( fx, fy): (usize, usize), input: &Vec<Vec<u8>>)->bool{
    //rust would think of this as risky I guess
    let height = input[fy][fx];
    for dir in &DIRECTIONS{
      let (mut x,mut y) = (fx as i32 + dir.0, fy as i32+dir.1);
        let mut is_visible = true;
        while is_in_bounds(x ,y ,input){
            if input[y as usize][x as usize] >= height{
                is_visible = false;
                break;
            }
            (x,y) = (x + dir.0, y+dir.1);
        }
        if is_visible{
            return true;
        }
    }
    false

}
fn check_score( ( fx, fy): (usize, usize), input: &Vec<Vec<u8>>)->u64{
    //rust would think of this as risky I guess
    let height = input[fy][fx];
    let mut final_score = 1;
    for dir in &DIRECTIONS{
        let mut score = 0;
        let (mut x,mut y) = (fx as i32 + dir.0, fy as i32+dir.1);
        while is_in_bounds(x ,y ,input){
            score += 1;
            if input[y as usize][x as usize] >= height{
                break;
            }
            (x,y) = (x + dir.0, y+dir.1);
        }
        final_score *= score;
    }
    final_score

}

fn first_part(input: &Vec<Vec<u8>>) -> u64 {
    let mut visible = 2 * (input.len() + (input[0].len() - 2)) as u64;
    for y in 1..(input.len()-1) {
        let row = &input[y];
        for x in 1..(row.len()-1) {
            if check_visibility((x, y), input) {
                visible += 1;
            };
        }
    }
    visible
}
fn second_part(input:&Vec<Vec<u8>>) -> u64 {
    let mut highest_score = 0;
    for y in 1..(input.len()-1) {
        let row = &input[y];
        for x in 1..(row.len()-1) {
            let score = check_score((x, y), input);
            highest_score = max(score, highest_score);
        }
    }
    highest_score

}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    println!("1. Test: {test_result}");
    let input_result = first_part(&input);
    println!("1. Input: {input_result}");
    let test_result = second_part(&test);
    println!("2. Test: {test_result}");
    let input_result = second_part(&input);
    println!("2. Input: {input_result}");
    // dbg!(test);
    println!("Hello, Rust");
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_check_inbounds(){
        let v = vec![vec![0; 5]; 5];
        assert_eq!(is_in_bounds(4, 3, &v), true);
    }
}