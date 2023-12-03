use std::fs;
use regex::Regex;

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
            x: if self.start.x == 0 { 0 } else { self.start.x - 1 },
            y: if self.start.y == 0 { 0 } else { self.start.y - 1 },
        };
        let br = Point { x: self.end.x + 1, y: self.end.y + 1 };
        ul.x <= p.x && p.x <= br.x && ul.y <= p.y && p.y <= br.y
    }

    fn within_points(&self, ps: &Vec<Point>) -> bool {
        ps.iter().any(|p| self.within_point(p))
    }
}

struct Number {
    n: u64,
    loc: Span,
}

fn main() {
    let number_re = Regex::new("\\d+").unwrap();
    let symbol_re = Regex::new("[^\\d.]").unwrap();

    let input = fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<Number> = input.split("\n").enumerate()
        .flat_map(|(line_nr, line)| number_re.find_iter(line).map(move |m| (line_nr, m)))
        .map(|(line_nr, num_match)| {
            Number {
                n: num_match.as_str().parse().unwrap(),
                loc: Span { start: Point { x: num_match.start(), y: line_nr }, end: Point { x: num_match.end() - 1, y: line_nr } },
            }
        }).collect();

    let symbols: Vec<Point> = input.split("\n").enumerate()
        .flat_map(|(line_nr, line)| symbol_re.find_iter(line).map(move |m| (line_nr, m)))
        .map(|(line_nr, sym_match)| Point { x: sym_match.start(), y: line_nr })
        .collect();

    let r: u64 = numbers.iter()
        .filter(|n| n.loc.within_points(&symbols))
        .map(|n| n.n)
        .sum();
    println!("{}", r);
}
