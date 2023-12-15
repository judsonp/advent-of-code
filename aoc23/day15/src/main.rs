use std::fs;

fn main() {
    let input_s = fs::read_to_string("inputs/day15.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &Vec<&str>) -> u64 {
    input.iter().map(|s| snow_hash(s) as u64).sum()
}

fn snow_hash(string: &str) -> u8 {
    string.chars().fold(0u8, |acc, c| hash_step(acc, c))
}

fn hash_step(init: u8, next: char) -> u8 {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.

    let mut state = init as u32;
    assert!(next.is_ascii());
    assert!((next as u32) < 128);
    state += next as u32;
    state *= 17;
    state %= 256;
    return state as u8;
}

fn parse_input(input: &str) -> Vec<&str> {
    return input.trim().split(",").collect();
}