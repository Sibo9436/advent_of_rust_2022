// Advent of code 2022
use std::collections::HashMap;

#[derive(PartialOrd, PartialEq,Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn calc_points(choice: Choice, outcome: Outcome) -> u64 {
    let val = match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissor => 3,
    };
    val + match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    }
}

fn first_part(input: &Vec<(char, char)>) -> u64 {
    let mut total_points = 0;
    for (opp, me) in input{
        let mychoice = match me {
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissor,
            _ => unreachable!("Argh"),
        };
        let outcome = find_outcome(*opp, *me);
        let points = calc_points(mychoice, outcome);
        total_points += points;
    }
    total_points
}
fn find_choice(desired_outcome: &Outcome, opp: char) -> Choice {
    let op_value = opp as u64 - 'A' as u64;
    let choice = match desired_outcome{
        Outcome::Win =>
            (op_value + 1) % 3,
        Outcome::Loss =>
            (op_value +2) %3,
        Outcome::Draw => op_value,
    };
    match choice {
        0 => Choice::Rock,
        1=> Choice::Paper,
        2=>Choice::Scissor,
        _=> unreachable!("NONE"),
    }
}
fn second_part(input: &Vec<(char,char)>) -> u64{
    let mut total_points = 0;
    for (opp, me) in input{
        let desired_outcome = match me{
            'X'=>  Outcome::Loss,
            'Y'=>  Outcome::Draw,
            'Z'=>  Outcome::Win,
            _ => unreachable!("Argh"),
        };
        let choice = find_choice(&desired_outcome, *opp);
        let points = calc_points(choice, desired_outcome);
        total_points += points;

    }
    total_points
}

fn find_outcome(opp: char, me: char) -> Outcome {
    let opp_val = opp as u64 - 'A' as u64;
    let me_val = me as u64 - 'X' as u64;
    if opp_val == me_val {return Outcome::Draw};

    if (opp_val +1)%3 == me_val {return Outcome::Win};
    Outcome::Loss
}

fn parse_input(input: &str) -> Vec<(char, char)> {
    input
        .trim()
        .split("\n")
        .map(|line| line.trim().split_once(" ").unwrap())
        .map(|(f, s)| {
            (
                f.chars().nth(0).expect("Something broken"),
                s.chars().nth(0).expect("Smth broke"),
            )
        })
        .collect()
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let input = parse_input(input);
    let test = parse_input(test);
    println!("FIRST PART");
    let test_result = first_part(&test);
    println!("Test result:  {test_result}");
    let input_result = first_part(&input);
    println!("Result: {input_result}");
    println!("SECOND PART");
    let test_result = second_part(&test);
    println!("Test result:  {test_result}");
    let input_result = second_part(&input);
    println!("Result: {input_result}");



    println!("Hello, Rust");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_points() {
        let result = calc_points(Choice::Paper, Outcome::Win);
        assert_eq!(result, 8);
        let result = calc_points(Choice::Rock, Outcome::Loss);
        assert_eq!(result, 1);
        let result = calc_points(Choice::Scissor, Outcome::Draw);
        assert_eq!(result, 6);
    }
    #[test]
    fn test_find_outcome(){
        let outcome = find_outcome('A', 'X');
        assert_eq!(outcome, Outcome::Draw);
        let outcome = find_outcome('A', 'Y');
        assert_eq!(outcome, Outcome::Win);
        let outcome = find_outcome('A', 'Z');
        assert_eq!(outcome, Outcome::Loss);

    }
}