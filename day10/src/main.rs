// Advent of code 2022

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::Operation::{AddX, NoOp};

#[derive(Debug,Copy,Clone)]
#[repr(usize)]
enum Operation {
    NoOp = 0_usize,
    AddX,
}
#[derive(Debug)]
struct Instruction{
    op: Operation,
    val: i32,
}
//POTREBBE ESSERE CARINO ASTRARRE LA "CPU" PER RENDERLA PIU' SIMILE A UN
//EMULATORE VERO E DISACCOPPIARE LE LOGICHE DI  CPU E CRT

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim() == "noop" {
            return Ok(Self{
                op:NoOp,
                val:0
            });
        }
        let value = s.trim().strip_prefix("addx ").ok_or(())?;
        match value.parse() {
            Ok(value) => Ok(Self {
                op: AddX,
                val:value
            }),
            Err(_) => return Err(())
        }
    }
}

const ticks: &[i32; 2] = &[1, 2];

fn first_part(input: &[Instruction]) -> i32 {
    let mut current_instruction = 0;
    let mut x_register = 1;
    let mut tick_counter = 1;
    let mut total = 0;
    let mut remaining_ticks:i32 = ticks[*&input[current_instruction].op as usize]-1;
    while current_instruction < input.len() {
        if (tick_counter -20) % 40 == 0{
            println!("Adding to total {} at tick {tick_counter}", tick_counter * x_register);
            total += tick_counter * x_register;
        }
        println!("[{tick_counter}]:[{x_register}] {:?} {remaining_ticks}", &input[current_instruction]);
        if remaining_ticks == 0 {
            match &input[current_instruction].op {
                NoOp => {}
                AddX=> { x_register += &input[current_instruction].val; }
            };
            current_instruction += 1;
            if current_instruction == input.len() { break; }
            remaining_ticks = ticks[*&input[current_instruction].op as usize]
        }
        remaining_ticks -= 1;

        tick_counter += 1;
    }
    println!("Final status: ticks: {tick_counter}, x:{x_register}");


    total
}
fn second_part(input: &Vec<Instruction>)-> String{
    let mut current_instruction = 0;
    let mut x_register = 1;
    let mut tick_counter:i32 = 1;
    let mut remaining_ticks:i32 = ticks[*&input[current_instruction].op as usize]-1;
    let mut crt_screen = vec! {vec! {'x'; 40}; 6};
    while current_instruction < input.len() {
        if ((tick_counter-1)%40).abs_diff(x_register)<=1{
            crt_screen[((tick_counter-1)/40) as usize][((tick_counter-1)%40) as usize] = '#';
        }else{
            crt_screen[((tick_counter-1)/40) as usize][((tick_counter-1)%40) as usize] = ' ';
        }
        println!("[{tick_counter}]:[{x_register}] {:?} {remaining_ticks}", &input[current_instruction]);
        if remaining_ticks == 0 {
            match &input[current_instruction].op {
                NoOp => {}
                AddX=> { x_register += &input[current_instruction].val; }
            };
            current_instruction += 1;
            if current_instruction == input.len() { break; }
            remaining_ticks = ticks[*&input[current_instruction].op as usize]
        }
        remaining_ticks -= 1;

        tick_counter += 1;
    }
    println!("Final status: ticks: {tick_counter}, x:{x_register}");

    let mut res = String::new();
    for row in crt_screen.iter(){
       for ch in row.iter(){
           res.push(*ch);
       }
        res.push('\n');
    }
    res
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().split('\n').map(|line| Instruction::from_str(line).unwrap()).collect()
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    println!("1. Test {test_result}");

    let input_result = first_part(&input);
    println!("1. Test {input_result}");
    let test_result = second_part(&test);
    let input_result = second_part(&input);
    println!("2. Input\n{input_result}");
    println!("Hello, Rust");
}
