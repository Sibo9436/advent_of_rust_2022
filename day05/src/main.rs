// Advent of code 2022

#[derive(Debug)]
struct Instruction {
    quantity: i32,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}
//Could be rewritten implementing fromstr, maybe later

fn parse_input(input: &str) -> Input {
    let (stacks, moves) = input.trim_end().split_once("\n\n").unwrap();
    let mut stacks: Vec<&str> = stacks.split('\n').rev().collect();
    let number_of_stacks = (stacks[0].trim_end().len() + 2) / 4;
    stacks.remove(0);
    let mut bins: Vec<Vec<char>> = vec![Vec::new(); number_of_stacks];
    for line in stacks {
        let mut index = 1;
        let mut vindex = 0;
        while index < line.len() {
            let ch = line.chars().nth(index).unwrap();
            if ch != ' ' {
                bins[vindex].push(ch);
            }
            index += 4;
            vindex += 1;
        }
    }

    let moves = moves
        .split('\n')
        .map(|line| {
            let r: Vec<&str> = line
                .trim()
                .split(' ')
                .filter(|w| !matches!(*w, "move" | "from" | "to"))
                .collect();
            if r.len() != 3 {
                panic!("NOOOOOOO")
            }
            Instruction {
                quantity: r[0].parse::<i32>().unwrap(),
                from: r[1].parse::<usize>().unwrap(),
                to: r[2].parse::<usize>().unwrap(),
            }
        })
        .collect();
    Input {
        stacks: bins,
        instructions: moves,
    }
}

fn first_part(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for inst in &input.instructions {
        for _ in 0..inst.quantity {
            let removed = stacks[inst.from - 1].pop().unwrap();
            stacks[inst.to - 1].push(removed);
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn second_part(input: &Input) -> String {
    let mut stacks = input.stacks.clone();
    for inst in &input.instructions {
        let split_at = stacks[inst.from - 1].len() - inst.quantity as usize;
        let split = &mut stacks[inst.from - 1][split_at..];
        let mut splitv: Vec<char> = Vec::new();
        split.clone_into(&mut splitv);
        stacks[inst.to - 1].append(&mut splitv);
        for _q in 0..inst.quantity {
            stacks[inst.from - 1].pop();
        }
    }

    stacks.iter().map(|v| v.last().unwrap()).collect()
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");

    let test = parse_input(test);
    println!("{:?}", test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    println!("1. Test result: {test_result}");
    let input_result = first_part(&input);
    println!("1. Input result: {input_result}");
    let test_result = second_part(&test);
    println!("2. Test result: {test_result}");
    let input_result = second_part(&input);
    println!("2. Input result: {input_result}");
    println!("Hello, Rust");
}
