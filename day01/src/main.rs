use std::cmp::max;

// Advent of code 2022
fn parse_input(input : &str)->Vec<Vec<u64>>{
    let input = input.trim();
    let input : Vec<Vec<u64>>= input.split("\n\n").map( | s | {
        s.trim().split("\n").map(|l| {u64::from_str_radix(l,10).unwrap()}).collect()
    }).collect();
    input
}

fn first_part(input : &Vec<Vec<u64>>)-> u64{
    let reduced : Vec<u64> = input.iter().map(|v| v.iter().fold(0, |acc, v| v+acc)).collect();
    dbg!(&reduced);
    let highest = reduced.iter().fold(0,|val,acc| max(val,*acc) );

    highest
}
fn second_part(input: &Vec<Vec<u64>>) -> u64{
    let mut  reduced : Vec<u64> = input.iter().map(|v| v.iter().fold(0, |acc, v| v+acc)).collect();
    reduced.sort();
    reduced.reverse();
    dbg!(&reduced);
    reduced[0] + reduced[1] + reduced[2]

}
fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let input_result = first_part(&input);
    println!("Risultato prima parte: {input_result}");
    let input_result = second_part(&input);
    println!("Risultato seconda parte: {input_result}");



    println!("Hello, Rust");
}
