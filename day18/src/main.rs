// Advent of code 2022

use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn to_truple(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    //I assumed the coordinates couldn't go below 0, I hope I assumed correctly
    let points: Vec<Point> = input.trim().split('\n')
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();
            Point {
                x,
                y,
                z,
            }
        }).collect();
    println!("{:?}", points);

    points
}

fn first_part(points: &Vec<Point>) -> u64 {
    //Let's see how bruteforcing does
    let mut surface_area = 0;
    let mut coords = HashSet::new();
    for point in points {
        coords.insert(point.to_truple());
    }
    for point in points {
        let (x, y, z) = (point.x, point.y, point.z);
        if !coords.contains(&(x, y, z + 1)) { surface_area += 1; }
        if !coords.contains(&(x, y, z - 1)) { surface_area += 1; }
        if !coords.contains(&(x, y + 1, z)) { surface_area += 1; }
        if !coords.contains(&(x, y - 1, z)) { surface_area += 1; }
        if !coords.contains(&(x + 1, y, z)) { surface_area += 1; }
        if !coords.contains(&(x - 1, y, z)) { surface_area += 1; }
    }
    surface_area
}

fn second_part(points: &Vec<Point>) -> u64 {
    let mut surface_area = first_part(points);
    let mut coords = HashSet::new();
    for point in points {
        coords.insert(point.to_truple());
    }
    let mut checked_sides = HashSet::new();
    let mut check_droplet = |(x, y, z): (i32, i32, i32)| {
        if !checked_sides.contains(&(x, y, z))
            && !coords.contains(&(x, y, z))
            && coords.contains(&(x, y, z + 1))
            && coords.contains(&(x, y, z - 1))
            && coords.contains(&(x, y + 1, z))
            && coords.contains(&(x, y - 1, z))
            && coords.contains(&(x + 1, y, z))
            && coords.contains(&(x - 1, y, z)) {
            println!("FOUND POCKET AT {x},{y},{z}");
            checked_sides.insert((x, y, z));
            return true;
        }
        checked_sides.insert((x, y, z));
        false
    };
    for point in points {
        let (x, y, z) = (point.x, point.y, point.z);
        if check_droplet((x, y, z + 1)) { surface_area -= 6; }
        if check_droplet((x, y, z - 1)) { surface_area -= 6; }
        if check_droplet((x, y + 1, z)) { surface_area -= 6; }
        if check_droplet((x, y - 1, z)) { surface_area -= 6; }
        if check_droplet((x + 1, y, z)) { surface_area -= 6; }
        if check_droplet((x - 1, y, z)) { surface_area -= 6; }
    }
    surface_area
}

fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    let test_result = first_part(&test);
    println!("1. Test {test_result}");
    let input_result = first_part(&input);
    println!("1. Input {input_result}");
    let test_result = second_part(&test);
    println!("1. Test {test_result}");
    let input_result = second_part(&input);
    println!("1. Input {input_result}");
    println!("Hello, Rust");
}
