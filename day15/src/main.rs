use std::cmp::{max, min};
use std::collections::HashSet;

// Advent of code 2022
#[derive(Debug)]
struct Sensor{
    x : i32,
    y : i32,
    closest_beacon: (i32,i32)
}
fn extract_coords(coords:&str) -> (i32, i32) {
    let (x, y) = coords.trim().split_once(',').unwrap();
    let x = x.trim().strip_prefix("x=").unwrap().parse().unwrap();
    let y = y.trim().strip_prefix("y=").unwrap().parse().unwrap();
    (x,y)

}
fn parse_input(input:&str)->Vec<Sensor>{
    input.trim().split('\n')
        .map(|line|{
            let (mut first, mut second) = line.split_once(':').unwrap();
            first = first.trim().strip_prefix("Sensor at").unwrap();
            second = second.trim().strip_prefix("closest beacon is at").unwrap();
            let position = extract_coords(first);
            Sensor{
                x: position.0,
                y:position.1,
                closest_beacon: extract_coords(second),
            }
        }).collect()
}
fn Old_first_part(input: &Vec<Sensor>, target_y : i32) -> u64 {
    //No beacons can be at the same distance from a sensor
    //I feel like this is quite the bruteforce way but alas
    let mut covered = HashSet::new();
    let mut beacons = HashSet::new();
    let mut sensors = HashSet::new();
    for sensor in input{
        sensors.insert((sensor.x, sensor.y));
        let torino_distance = (sensor.x - sensor.closest_beacon.0).abs() + (sensor.y - sensor.closest_beacon.1).abs();
        println!("{:?}\nDistance from beacon: {torino_distance} ",sensor);
        let ( x,  y) = (sensor.x, sensor.y );
        beacons.insert((sensor.closest_beacon.0, sensor.closest_beacon.1));

        let mut x_count = 0;
        for ny in (y-torino_distance)..=y{
            for nx in (x - x_count)..=(x + x_count) {
                covered.insert((nx, ny));
            }
            x_count+=1;
        }
        x_count -= 1;
        for ny in y..=(y+torino_distance){
            for nx in (x - x_count)..=(x + x_count) {
                covered.insert((nx, ny));
            }
            x_count -= 1;
        }
    }
    let mut count = 0;
    for (x,y) in &covered{
        if *y == target_y && !beacons.contains(&(*x,*y)) && !sensors.contains(&(*x,*y)){
            println!("{x}:{y}");
            count +=1;
        }
    }
    for y in -2..22{
        for x in -2..=25{
            if beacons.contains(&(x, y)){
                print!("B");
            }else if sensors.contains(&(x, y)){
                print!("S");
            }else if covered.contains(&(x, y)){
                print!("#");
            }else{
                print!(".");
            }
        }
        print!("\n");
    }
    count

}
fn first_part(input: &Vec<Sensor>, target_y: i32)->u64{
    let mut distances = vec![0; input.len()];
    let mut rightmost_covered = 0;
    let mut leftmost_covered = 0;
    let mut beacons = HashSet::new();
    let mut sensors = HashSet::new();
    for i in 0..input.len() {
        let sensor = &input[i];
        sensors.insert((sensor.x, sensor.y));
        beacons.insert((sensor.closest_beacon.0, sensor.closest_beacon.1));
        let dist = (sensor.x - sensor.closest_beacon.0).abs() + (sensor.y - sensor.closest_beacon.1).abs();
        distances[i] = dist;
        leftmost_covered = min(leftmost_covered, sensor.x - dist);
        rightmost_covered = max(rightmost_covered, sensor.x + dist);
    };
    println!("{:?}", distances);
    println!("{leftmost_covered}->{rightmost_covered}");
    let mut count = 0;
    for x in leftmost_covered..=rightmost_covered{
        for (i,sensor) in input.iter().enumerate(){
            let dist = (sensor.x - x).abs() + (sensor.y - target_y).abs();
            if dist <= distances[i] && !beacons.contains(&(x, target_y)) && !sensors.contains(&(x, target_y)){
                count+=1;
                break;
            }
        }
    }
    println!("There's {}, different beacons", beacons.len());
    count
}
fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    let test = parse_input(test);
    let input = parse_input(input);
    println!("{:?}", test);
    let test_result = first_part(&test, 10);
    println!("1. Test {test_result}");
    let test_result = first_part(&input, 2000000);
    println!("1. Input {test_result}");

    println!("Hello, Rust");
}
