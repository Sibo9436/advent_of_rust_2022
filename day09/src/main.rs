// Advent of code 2022

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum Dir {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Dir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, s) = s.trim().split_once(' ').ok_or(())?;
        let s = match s.
            parse() {
            Ok(v) => v,
            Err(_) => return Err(()),
        };
        match d {
            "U" => Ok(Dir::Up(s)),
            "D" => Ok(Dir::Down(s)),
            "R" => Ok(Dir::Right(s)),
            "L" => Ok(Dir::Left(s)),
            _ => Err(())
        }
    }
}

fn parse_input(input: &str) -> Vec<Dir> {
    input.trim().split('\n').map(|l| Dir::from_str(l).unwrap()).collect()
}

fn first_part(input: &[Dir]) -> u64 {
    solution(input, 2)
}
fn second_part(input:&[Dir]) -> u64 {
    solution(input, 10)
}


fn solution(input: &[Dir],rope_len:usize) -> u64 {
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_len];
    let mut positions = HashSet::new();
    positions.insert(rope[rope_len-1]);
    for dir in input {
        let (d, steps) = match dir {
            Dir::Up(s) => ((0, 1), *s),
            Dir::Down(s) => ((0, -1), *s),
            Dir::Left(s) => ((-1, 0), *s),
            Dir::Right(s) => ((1, 0), *s),
        };
        for _ in 0..steps {
            rope[0] = (rope[0].0 + d.0, rope[0].1 + d.1);
            for i in 0..rope.len() - 1 {
                let vdist = rope[i].1 - rope[i + 1].1;
                let hdist = rope[i].0 - rope[i + 1].0;
                if hdist.abs() > 1 || vdist.abs() > 1 {
                    {
                        if hdist != 0 { rope[i + 1].0 += hdist / hdist.abs() };
                        if vdist != 0 { rope[i + 1].1 += vdist / vdist.abs() };
                    }
                }
            }
            // print_rope(&rope);
            positions.insert(rope[rope_len-1]);
        }
    }
    // dbg!(&positions);
    positions.len() as u64
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");

    let test = parse_input(test);
    let input = parse_input(input);
    // dbg!(test);
    let test_result = first_part(&test);
    let input_result = first_part(&input);
    println!("1. Test {test_result}");
    println!("1. Input {input_result}");

    let test_result = second_part(&test);
    let input_result = second_part(&input);
    println!("2. Test {test_result}");
    println!("2. Input {input_result}");

    println!("Hello, Rust");
}
