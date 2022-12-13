use std::cmp::{min, Ordering};
use std::collections::BinaryHeap;

// Advent of code 2022
#[derive(Debug)]
struct Input {
    start: (usize, usize),
    end: (usize, usize),
    height_map: Vec<Vec<u8>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Step {
    x: usize,
    y: usize,
    dist: u64,
}

//I have to implement Ord and PartialOrd for it to work with the heap
impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist).then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn parse_input(input: &str) -> Input {
    let mut res = Input {
        start: (0, 0),
        end: (0, 0),
        height_map: vec![],
    };
    for (y, line) in input.trim().split('\n').enumerate() {
        res.height_map.push(vec![]);
        for (x, ch) in line.chars().enumerate() {
            res.height_map.last_mut().unwrap().push(
                match ch {
                    'S' => {
                        res.start = (x, y);
                        0
                    }
                    'E' => {
                        res.end = (x, y);
                        25
                    }
                    _ => ch as u8 - b'a'
                }
            )
        }
    }
    res
}


fn first_part(input: &Input) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    heap.push(Step {
        x: input.end.0,
        y: input.end.1,
        dist: 0,
    });
    let map_width = input.height_map[0].len();
    let map_height = input.height_map.len();
    let mut shortest_distances = vec![u64::MAX; map_height * map_width];
    while let Some(Step { x, y, dist }) = heap.pop() {
        if x == input.start.0 && y == input.start.1 {
            return Some(dist);
        }
        if dist > shortest_distances[map_width * y + x] { continue; }
        //I'm pretty sure the second part will be using height difference as edge weight
        for (nbx, nby) in reachable_neighbors(x, y, &input.height_map) {
            let nb = Step {
                x: nbx,
                y: nby,
                dist: dist + 1,
            };
            if nb.dist < shortest_distances[map_width * nby + nbx] {
                heap.push(nb);
                shortest_distances[map_width * nby + nbx] = nb.dist;
            }
        };
    }
    None
    // unreachable!("I prefer this rather than Some or None just because this is problem solving code, not a production grade library")
}
fn second_part(input: &Input) -> u64 {
    let mut starting_from = Vec::new();
    for row in input.height_map.iter().enumerate(){
        for h in row.1.iter().enumerate() {
            if h.1 == &0 {
                starting_from.push((h.0, row.0));
            }
        }
    }
    starting_from.iter().map(|start|{
        Input{
            start: (start.0,start.1),
            end: input.end,
            height_map: input.height_map.clone(),
        }
    }).map(|input| first_part(&input)).filter(|o| o.is_some())
        .map(|op|op.unwrap())
        .fold(u64::MAX,|a,b| min(a,b))
}

fn reachable_neighbors(px: usize, py: usize, map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    //since I'm using usizes the lower bounds get "auto-adjusted"
    let mut rn = Vec::new();
    for y in (py.saturating_sub(1))..=(py + 1) {
        if y == py || y>=map.len(){ continue; }
        if map[y][px] + 1 >= map[py][px] {
            rn.push((px, y));
        }
    }
    for x in (px.saturating_sub(1))..=(px + 1) {
        if x == px || x >= map[0].len(){ continue; }
        if map[py][x] + 1 >= map[py][px] {
            rn.push((x, py));
        }
    }
    rn
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);

    let test_result = first_part(&test).unwrap();
    println!("1. Test {test_result}");
    let input_result = first_part(&input).unwrap();
    println!("1. Input {input_result}");
    let test_result = second_part(&test);
    println!("1. Test {test_result}");
     let input_result = second_part(&input);
    println!("1. Test {input_result}");
    println!("Hello, Rust");
}
