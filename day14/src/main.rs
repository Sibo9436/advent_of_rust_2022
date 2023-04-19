use std::cmp::max;
use std::collections::HashSet;

// Advent of code 2022
fn parse_input(input:&str)->Vec<Vec<(i32,i32)>>{
    input.trim().split('\n').map(|line|{
        line.split("->").map(
            |pair|{
                let p = pair.trim().split_once(',').unwrap();
                (p.0.parse().unwrap(),p.1.parse().unwrap())
            }
        ).collect()
    }).collect()
}

fn first_part(input:&Vec<Vec<(i32,i32)>>)->u64{
    let mut blocked_coords = HashSet::new();
    let mut lowest_rock = 0;//We count height "backwards"
    for path in input{
        for i in 1..path.len(){
            let (mut x, mut y) = path[i-1];
            lowest_rock = max(lowest_rock, y);
            let second = path[i];
            lowest_rock = max(lowest_rock, second.1);
            blocked_coords.insert((second.0, second.1));
            if x == second.0{
                while y != second.1{
                    println!("LOOOPIN {y},{}",second.1);
                    blocked_coords.insert((x,y));
                    y += -(y -second.1).signum()
                }
            }else{
                while x != second.0{
                    blocked_coords.insert((x,y));
                    x += -(x-second.0).signum()
                }
            }
        }
    }
    println!("Lowest blocked point is: {lowest_rock}");
    for y in 0..10{
        for x in 494..=503{
            print!("{}",if blocked_coords.contains(&(x,y)){'#'}else{'.'})
        }
        print!("\n")
    }
    let mut count = 0;
    loop  {
        println!("inserting a sand grain");
        let (mut x, mut y) = (500, 0);
        let has_stopped = loop{
            println!("falling at {x}.{y}");
            if !blocked_coords.contains(&(x,y+1)){
                y +=1;
            }else if !blocked_coords.contains(&(x-1,y+1)){
                x -= 1;
                y+=1;
            }else if !blocked_coords.contains(&(x+1,y+1)){
                x += 1;
                y+=1;
            }else{
                //cannot move
                blocked_coords.insert((x,y));
                break true;
            }
            if y > lowest_rock{
                break false;
            }
        };
        if !has_stopped{break;}
        count+=1
    };
    count
}
fn second_part(input:&Vec<Vec<(i32,i32)>>)->u64{
    let mut blocked_coords = HashSet::new();
    let mut lowest_rock = 0;//We count height "backwards"
    for path in input{
        for i in 1..path.len(){
            let (mut x, mut y) = path[i-1];
            lowest_rock = max(lowest_rock, y);
            let second = path[i];
            lowest_rock = max(lowest_rock, second.1);
            blocked_coords.insert((second.0, second.1));
            if x == second.0{
                while y != second.1{
                    println!("LOOOPIN {y},{}",second.1);
                    blocked_coords.insert((x,y));
                    y += -(y -second.1).signum()
                }
            }else{
                while x != second.0{
                    blocked_coords.insert((x,y));
                    x += -(x-second.0).signum()
                }
            }
        }
    }
    println!("Lowest blocked point is: {lowest_rock}");
    for y in 0..10{
        for x in 494..=503{
            print!("{}",if blocked_coords.contains(&(x,y)){'#'}else{'.'})
        }
        print!("\n")
    }
    let mut count = 0;
    let plane = lowest_rock + 2;
    let start = (500, 0);
    loop  {
        // println!("inserting a sand grain");
        let (mut x, mut y) =start;
        let has_stopped = loop{
            // println!("falling at {x}.{y}");
            if y+1==plane{
                blocked_coords.insert((x,y));
                break (x, y);
            }
            else if !blocked_coords.contains(&(x,y+1)) {
                y +=1;
            }else if !blocked_coords.contains(&(x-1,y+1)){
                x -= 1;
                y+=1;
            }else if !blocked_coords.contains(&(x+1,y+1)){
                x += 1;
                y+=1;
            }else{
                //cannot move
                blocked_coords.insert((x,y));
                break (x,y);
            }
        };
        count += 1;
        if has_stopped == start{ break;}
    };
    count
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    println!("{:?}",test);
    let test_result = first_part(&test);
    println!("1. Test {test_result}");
     let input_result = first_part(&input);
    println!("1. Input {input_result}");
    let test_result = second_part(&test);
    println!("2. Test {test_result}");
    let input_result = second_part(&input);
    println!("2. Input {input_result}");
    println!("Hello, Rust");
}
