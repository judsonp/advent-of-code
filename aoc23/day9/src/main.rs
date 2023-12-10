use std::fs;
use nom::character::complete::{i64 as pi64, line_ending, space1};
use nom::IResult;
use nom::multi::separated_list1;

fn main() {
    let input_s = fs::read_to_string("inputs/day9.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(prediction).sum()
}

fn part_two(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(prediction2).sum()
}

fn prediction(input: &Vec<i64>) -> i64 {
    let mut seq = input.clone();
    let mut result: i64 = *seq.last().unwrap();
    let mut all_zero: bool = false;
    while !all_zero {
        all_zero = true;
        for idx in 1..seq.len() {
            seq[idx - 1] = seq[idx] - seq[idx - 1];
            if seq[idx - 1] != 0 {
                all_zero = false;
            }
        }
        seq.pop();
        result += seq.last().unwrap();
    }
    return result;
}

fn prediction2(input: &Vec<i64>) -> i64 {
    let mut seq = input.clone();
    let mut result: i64 = *seq.first().unwrap();
    let mut sign = -1;
    let mut all_zero: bool = false;
    while !all_zero {
        all_zero = true;
        for idx in 1..seq.len() {
            seq[idx - 1] = seq[idx] - seq[idx - 1];
            if seq[idx - 1] != 0 {
                all_zero = false;
            }
        }
        seq.pop();
        result += seq.first().unwrap() * sign;
        sign *= -1;
    }
    return result;
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i64>>>{
    separated_list1(line_ending, separated_list1(space1, pi64))(input)
}