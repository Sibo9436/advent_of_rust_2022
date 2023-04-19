// Advent of code 2022
#[derive(Debug,Clone)]
enum Item {
    Int(u64),
    List(Vec<Item>),
}
#[derive(Debug,Clone)]
struct Pair{
    left:Item,
    right:Item
}
enum Token{
    OpenBracket,
    ClosedBracket,
    Comma,
    Integer(String)
}
enum Truple{
    True,
    False,
    Continue
}

use std::cmp::Ordering;
use std::ops::{Deref, Index};
use crate::Item::*;
use crate::Token::*;
use crate::Truple::{Continue, False, True};

impl Item{
    fn is_list(&self)->bool{
        return matches!(self, List(_))
    }
    //this is hacky

    fn val(&self)->u64 {
        match self {
            Int(v) => {*v}
            List(v) => {panic!("Preposterous indeed")}
        }
    }

    fn len(&self)->usize{
        match self {
            Int(_) => {dbg!(self);panic!("Asking the lenght of an integer, how preposterous")}
            List(v) => {v.len()}
        }
    }
}
fn parse_line(input:&str)->Vec<Item>{
    let chars: Vec<char> = input.trim().chars().collect();
    let mut tokens = Vec::new();
    for mut index in 0..input.len() {
        tokens.push(match chars[index] {
            '[' => OpenBracket,
            ']' => ClosedBracket,
            ',' => Comma,
            _ => {
                let mut st = String::new();
                while chars[index].is_numeric() {
                    st.push(chars[index]);
                    index += 1;
                }
                Integer(st)
            }
        });
    }
    let mut list_stack:Vec<Item> = Vec::new();
    for token in tokens{
        match token{
            OpenBracket => {
                list_stack.push(List(Vec::new()));
            }
            ClosedBracket => {
                if list_stack.len()==1{
                    //It's the outermost list
                    break;
                }
                let it = list_stack.pop().unwrap();
                if let List(_) = it{
                   if let List(v) = list_stack.last_mut().unwrap(){
                       v.push(it)
                   } else{
                       panic!("I should be pushin'")
                   }
                }else{
                    panic!("There shouldn't be Integers ")
                }
            }
            Comma => { continue;}
            Integer(s) => {
                let mut last = list_stack.last_mut().unwrap();
                if let List(v) = last{
                    v.push(Int(s.parse().unwrap()));
                }else{
                    panic!("WHy");
                }
            }
        }
    }
    if list_stack.len() != 1{
        println!("Something wonky might have happened");
        dbg!(&list_stack);
    }
    let res = list_stack.get(0).unwrap();
    if let List(vec) = res{
        vec.clone()
    }else{
        panic!("Enough with this panic")
    }
}
fn parse_pair(input: &str)->Pair {
    let (left, right) = input.trim().split_once('\n').unwrap();
    Pair {
        left: List(parse_line(left).clone()),
        right: List(parse_line(right).clone())
    }
}
fn parse_input(input:&str)->Vec<Pair>{
    input.trim().split("\n\n")
        .map(parse_pair).collect()
}

fn first_part(input: &[Pair])->usize{
    let mut result = 0;
    for (i,p) in input.iter().enumerate(){

        match compare_pair(p){
            //In the solution it starts counting from one
            Truple::True  => {
                println!("Y:Pair [{}]{:?} is in order",i+1, p);
                result += i+1;
            }
            _ => {
                println!("N:Pair [{}]{:?} is not in order",i+1, p);
            }
        }
    }
    result
}


fn compare_pair(p: &Pair) -> Truple {
    let mut index = 0;
    //I could have gone with an option but eeeeh
    let leftlen = p.left.len();
    let rightlen = p.right.len();
    let left = if let  List(v) = &p.left { v } else { panic!("Why") };
    let right = if let List(v) = &p.right { v } else { panic!("Why") };

    //I also could implement cmp and partialcmp, but I'm not sure it'd be worth it
    while index < leftlen && index < rightlen{
        // println!("Comparing {:?} and {:?}", left[index], right[index]);
        let res = if left[index].is_list() && right[index].is_list(){
             compare_pair(&Pair{
                left:left[index].clone(),
                right:right[index].clone()
            })
        }else if left[index].is_list() && !right[index].is_list() {
            compare_pair(&Pair{
                left:left[index].clone(),
                right:List(vec![Int(right[index].val())])
            })
        }else if !left[index].is_list() && right[index].is_list(){
            compare_pair(&Pair{
                left:List(vec![Int(left[index].val())]),
                right:right[index].clone(),
            })
        }else{
            match left[index].val().cmp(&right[index].val()){
                Ordering::Less => {True}
                Ordering::Equal => {Continue}
                Ordering::Greater => {False}
            }
        };
        match res {
            True => {
                println!("True because \n{:?}\n{:?}\nare in order", left[index], right[index]);
                return True;
            }
            False => {return False}
            Continue => {
                index += 1;
                continue;}
        }
    }
    match leftlen.cmp(&rightlen) {
        Ordering::Less => {
            println!("True because \n{:?}\n{:?}\nand left is shorter: l:{leftlen},r:{rightlen}", left, right );
            True
        }
        Ordering::Equal => {
            println!("Continue because \n{:?}\n{:?}\nare of same lenght : l:{leftlen},r:{rightlen}", left, right );

            Continue
        }
        Ordering::Greater => {False}
    }
}
#[derive(Debug)]
struct Packet{
    item:Item,
    is_divisor:bool
}
fn second_part(input: &[Pair])->usize{
    let mut vector_of_distress:Vec<Packet> = input.iter().map(|p| vec![Packet{ item:p.left.clone(),is_divisor:false}, Packet{item:p.right.clone(),is_divisor:false}]).flatten().collect();
    println!("{:?}",vector_of_distress);
    vector_of_distress.push(Packet{
        item: List(vec![List(vec![Int(6)])]),
        is_divisor:true
    });
    vector_of_distress.push(Packet{
        item: List(vec![List(vec![Int(2)])]),
        is_divisor:true
    });

    vector_of_distress.sort_by(|a,b|{
        match compare_pair(&Pair{
            left:a.item.clone(),
            right:b.item.clone()
        }) {
            True => {Ordering::Less}
            False => {Ordering::Greater}
            Continue => {Ordering::Greater}
        }
    });
    let mut res = 1;
    for (i,line) in vector_of_distress.iter().enumerate(){
        if line.is_divisor{
            res *= i+1;
        }
        println!("[{}]{:?}", i+1,line);
    }
res
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    // dbg!(&test);
    let input = parse_input(input);
    // dbg!(&input);
    let test_result = first_part(&test);
    println!("1. Test {test_result}");
    let input_result = first_part(&input);
    println!("1. Input {input_result}");

    let test_result = second_part(&test);
    println!("Hello, Rust");
    let input_result = second_part(&input);
    println!("1. Input {input_result}");

}
