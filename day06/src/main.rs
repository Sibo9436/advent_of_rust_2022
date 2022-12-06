// Advent of code 2022
//I'm pretty sure there's a more elegant solution that keeps a
//running hash/map/set/something and updates it
//but it's late and I'm tired, sorry
fn find_substring(input: &str, size: usize) -> u64 {
    {
        let mut start_index = 0;
        let mut end_index = size - 1;
        while end_index != input.len() - 1 {
            let window = &input[start_index..=end_index];
            let mut window_hash = 0;
            let mut has_double = false;
            for ch in window.chars() {
                let slide: u32 = 1 << (ch as u8 - b'a');
                if window_hash & slide == 0 {
                    window_hash |= slide;
                } else {
                    has_double = true;
                }
            }
            if !has_double {
                return 1 + end_index as u64;
            }
            start_index += 1;
            end_index += 1;
        }
        unreachable!("WHY")
    }
}
fn first_part(input: &str) -> u64 {
    find_substring(input, 4)
}
fn second_part(input: &str) -> u64 {
    find_substring(input, 14)
}
fn main() {
    let test = include_str!("../input/test.txt");
    let input = include_str!("../input/input.txt");
    //not much parsing to do today
    let test_result = first_part(test);
    println!("1. Test: {test_result}");
    let input_result = first_part(input);
    println!("1. Input: {input_result}");
    let test_result = second_part(test);
    println!("2. Test: {test_result}");
    let input_result = second_part(input);
    println!("2. Input: {input_result}");

    println!("Hello, Rust");
}
