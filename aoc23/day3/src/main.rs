use regex::Regex;
use std::fs;

struct Point {
    x: usize,
    y: usize,
}

struct Span {
    start: Point,
    end: Point,
}

impl Span {
    fn within_point(&self, p: &Point) -> bool {
        let ul = Point {
            x: if self.start.x == 0 {
                0
            } else {
                self.start.x - 1
            },
            y: if self.start.y == 0 {
                0
            } else {
                self.start.y - 1
            },
        };
        let br = Point {
            x: self.end.x + 1,
            y: self.end.y + 1,
        };
        ul.x <= p.x && p.x <= br.x && ul.y <= p.y && p.y <= br.y
    }

    fn within_symbols<'a, I>(&self, ps: I) -> bool
    where
        I: IntoIterator<Item = &'a Symbol>,
    {
        ps.into_iter().any(|s| self.within_point(&s.loc))
    }
}

struct Number {
    n: u64,
    loc: Span,
}

struct Symbol {
    is_gear: bool,
    loc: Point,
}

fn main() {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let (numbers, symbols) = parse_input(&input);

    println!("Part 1: {}", part_one(&numbers, &symbols));
    println!("Part 2: {}", part_two(&numbers, &symbols));
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let number_re = Regex::new("\\d+").unwrap();
    let symbol_re = Regex::new("[^\\d.]").unwrap();

    let numbers: Vec<Number> = input
        .split("\n")
        .enumerate()
        .flat_map(|(line_nr, line)| number_re.find_iter(line).map(move |m| (line_nr, m)))
        .map(|(line_nr, num_match)| Number {
            n: num_match.as_str().parse().unwrap(),
            loc: Span {
                start: Point {
                    x: num_match.start(),
                    y: line_nr,
                },
                end: Point {
                    x: num_match.end() - 1,
                    y: line_nr,
                },
            },
        })
        .collect();

    let symbols: Vec<Symbol> = input
        .split("\n")
        .enumerate()
        .flat_map(|(line_nr, line)| symbol_re.find_iter(line).map(move |m| (line_nr, m)))
        .map(|(line_nr, sym_match)| Symbol {
            is_gear: sym_match.as_str().contains('*'),
            loc: Point {
                x: sym_match.start(),
                y: line_nr,
            },
        })
        .collect();

    return (numbers, symbols);
}

fn part_one(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u64 {
    numbers
        .iter()
        .filter(|n| n.loc.within_symbols(symbols))
        .map(|n| n.n)
        .sum()
}

fn part_two(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u64 {
    symbols
        .iter()
        .filter(|s| s.is_gear)
        .filter_map(|s| {
            let (count, gear_ratio) = numbers
                .iter()
                .filter(|n| n.loc.within_point(&s.loc))
                .map(|s| (1, s.n))
                .fold((0, 1), |(ac, ap), (c, p)| (ac + c, ap * p));
            if count == 2 {
                Some(gear_ratio)
            } else {
                None
            }
        })
        .sum()
}
