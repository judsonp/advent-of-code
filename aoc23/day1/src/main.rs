#[macro_use]
extern crate lazy_static;

use num2words::{Lang, Num2Words};
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref DIGITS: HashMap<String, u32> = {
        let mut map = HashMap::new();
        for i in 1..=9 {
            map.insert(i.to_string(), i);
            map.insert(Num2Words::new(i).lang(Lang::English).to_words().unwrap(), i);
        }
        map
    };
}

fn value_two(line: &str) -> u32 {
    let (first, last) = (0..line.len())
        .filter_map(|start| {
            DIGITS
                .iter()
                .flat_map(|(s, d)| line[start..].starts_with(s).then_some(d))
                .next()
        })
        .copied()
        .fold((None, None), |(f, _), x| (f.or(Some(x)), Some(x)));
    first.unwrap() * 10 + last.unwrap()
}

fn value_one(line: &str) -> u32 {
    let (first, last) = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold((None, None), |(f, _), x| (f.or(Some(x)), Some(x)));
    first.unwrap() * 10 + last.unwrap()
}

fn part_two(lines: &[&str]) -> u32 {
    lines.iter().map(|line| value_two(line)).sum()
}

fn part_one(lines: &[&str]) -> u32 {
    lines.iter().map(|line| value_one(line)).sum()
}

fn main() {
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    println!("Part one: {}", part_one(&lines));
    println!("Part two: {}", part_two(&lines));
}
