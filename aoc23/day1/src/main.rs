#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num2words::{Num2Words, Lang};

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
            DIGITS.iter()
                .flat_map(|(s, d)| line[start..].starts_with(s).then_some(d))
                .next()
        })
        .copied()
        .fold((None, None), |(f, _), x| (f.or(Some(x)), Some(x)));
    first.unwrap() * 10 + last.unwrap()
}

fn value_one(line: &str) -> u32 {
    let (first, last) = line.chars()
        .filter_map(|c| c.to_digit(10))
        .fold((None, None), |(f, _), x| (f.or(Some(x)), Some(x)));
    first.unwrap() * 10 + last.unwrap()
}

fn part_two(lines: io::Lines<io::BufReader<File>>) -> u32 {
    lines.map(|line| value_two(&line.unwrap()))
        .sum()
}

fn part_one(lines: io::Lines<io::BufReader<File>>) -> u32 {
    lines.map(|line| value_one(&line.unwrap()))
        .sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let lines = read_lines("input.txt").unwrap();
    println!("{}", part_two(lines));
}
