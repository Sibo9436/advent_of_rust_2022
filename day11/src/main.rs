// Advent of code 2022

extern crate core;


//If I had the time I'd take it to implement a nice parser with a tokenizer, lexer and everything
//that'd be swell
//maybe I might come back later for this but for now it's not on the menu, sorry
#[derive(Debug,Clone)]
struct Target{
    vero: usize,
    falso:usize,
}

#[derive(Debug, Default,Clone)]
struct Operation{
    //new = lhs op rhs
    lhs : String,
    op : String,
    rhs: String,
}
#[derive(Debug,Clone)]
struct Monkey {
    items : Vec<i64>,
    operation: Operation,
    test: i64, //seems to always be "divisible by"
    target: Target,
}

//I'm not very proud of this...
fn parse_input(input:&str)->Vec<Monkey>{
    input.trim().split("\n\n")
        .map(|blob|{
            let mut operation= Operation::default();
            let mut items :Vec<i64> = Vec::new();
            let mut test = 0;
            let mut target = Target { vero: 0, falso: 0 };
            for line in blob.split('\n'){
                //I'm sure there's better ways
                let line = line.trim();
                if line.starts_with("Starting items"){
                    let (_, i) = line.split_once(':').unwrap();
                    items = i.trim().split(',').map(|t| t.trim().parse().unwrap()).collect();
                }
                if line.starts_with("Operation"){
                    let (_, i) = line.split_once(':').unwrap();
                    let mut  i = i.split_once('=').unwrap().1.trim().split(' ');

                    operation = Operation{
                        lhs: i.next().unwrap().into(),
                        op: i.next().unwrap().into(),
                        rhs: i.next().unwrap().into(),
                    }
                }
                if line.starts_with("Test"){
                    let (_, i) = line.split_once(':').unwrap();
                    test = i.split_once("divisible by ").unwrap().1.parse().unwrap();
                }
                if line.starts_with("If"){
                    let (cond, mon) = line.split_once(':').unwrap();
                    let mon:usize = mon.split_once("throw to monkey").unwrap().1.trim().parse().unwrap();
                    match cond{
                        "If true" => target.vero = mon,
                        "If false"=> target.falso = mon,
                        _=>unreachable!("WHYYY")
                    }
                }
            }
            Monkey{
                items,
                operation,
                test,
                target,
            }

        }).collect()
}

fn first_part(input: &[Monkey]) -> i64 {
    //I make a vec so I can modify it and still reuse the input for part 2;
    let mut monkeys = input.to_vec();
    let mut monkey_inspections = vec![0; input.len()];
    for _ in 0..20{
        for index in 0..monkeys.len(){
            while monkeys[index].items.len() > 0{
                let mut monkey = &mut monkeys[index];
                monkey_inspections[index]+=1;
                let val = monkey.items.remove(0);
                //It's a shame I do this every iteration
                let lhs = match monkey.operation.lhs.as_str(){
                    "old" => val,
                    _=>monkey.operation.lhs.parse::<i64>().unwrap(),
                };
                 let rhs = match monkey.operation.rhs.as_str(){
                    "old" => val,
                    _=>monkey.operation.rhs.parse().unwrap(),
                };
                let val = match monkey.operation.op.as_str() {
                    "+"=>lhs+rhs,
                    "*"=>lhs*rhs,
                    _=>unreachable!("PORQUEEE")
                };
                let final_value = val/3;
                let target = if final_value % monkey.test == 0{
                    monkey.target.vero
                }else{
                    monkey.target.falso
                };
                monkeys[target].items.push(final_value);
            }
        }
    }
    monkey_inspections.sort_by(|a,b| {b.cmp(a)});
    // monkey_inspections.reverse()
    dbg!(&monkey_inspections);
    monkey_inspections[0] * monkey_inspections[1]
}
fn second_part(input: &[Monkey]) -> i64 {
    //I make a vec so I can modify it and still reuse the input for part 2;
    let mut monkeys = input.to_vec();
    let mut moduli = Vec::new();
    let mut itemcount=0;
    for monkey in &monkeys{
        moduli.push(monkey.test);
        itemcount += monkey.items.len();
    }
    let mut item_pool: Vec<Vec<i64>> = vec![Vec::new(); itemcount];
    let mut item_index = 0;
    for (index,monkey) in monkeys.iter_mut().enumerate(){
        for mut item in &mut monkey.items{
            //we remove the actual value and insert a reference to our item index
            let val = *item;
            *item = item_index as i64;
            for modulo in &moduli{
                item_pool[item_index].push(val % modulo);
            }
            item_index+=1;
        }
    }
    // dbg!(&item_pool);
    // dbg!(&moduli);
    // dbg!(&monkeys);
    let mut monkey_inspections = vec![0; input.len()];
    for _ in 0..10000{
        // dbg!(&monkeys);
        for index in 0..monkeys.len(){
            while monkeys[index].items.len() > 0{
                let mut monkey = &mut monkeys[index];
                let test = monkey.test;
                monkey_inspections[index]+=1;
                let item_index = monkey.items.remove(0);
                //It's a shame I do this every iteration
                //And now I do this for every monke
                for (midx,modulo) in moduli.iter().enumerate(){
                    let val = item_pool[item_index as usize][midx];
                    let lhs = match monkey.operation.lhs.as_str(){
                        "old" => val,
                        _=>monkey.operation.lhs.parse::<i64>().unwrap(),
                    };
                    let rhs = match monkey.operation.rhs.as_str(){
                        "old" => val,
                        _=>monkey.operation.rhs.parse::<i64>().unwrap(),
                    };
                    let final_value = match monkey.operation.op.as_str() {
                        "+"=>(lhs+rhs)%modulo,
                        "*"=>(lhs*rhs)%modulo,
                        _=>unreachable!("PORQUEEE")
                    };
                    item_pool[item_index as usize][midx] = final_value;
                }

                let target = if item_pool[item_index as usize][index]== 0{
                    monkey.target.vero
                }else{
                    monkey.target.falso
                };
                let other = monkeys[target].test;
                // println!("Pushing  value {} to monkey {target}, val:{val}, final:{final_value}, test:{test}", val % other);
                monkeys[target].items.push(item_index);
            }
        }
    }
    monkey_inspections.sort_by(|a,b| {b.cmp(a)});
    // monkey_inspections.reverse()
    dbg!(&monkey_inspections);
    monkey_inspections[0] * monkey_inspections[1]
}
fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");

    let test = parse_input(test);
    let input = parse_input(input);
    dbg!(&test);
    let test_result = first_part(&test);
    println!("1.Test {test_result}");
    let input_result = first_part(&input);
    println!("1.Input {input_result}");
    let test_result = second_part(&test);
    println!("2.Test {test_result}");
    let input_result = second_part(&input);
    println!("2.Input {input_result}");

    println!("Hello, Rust");
}
