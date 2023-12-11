use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use derive_more::Constructor;
use itertools::Itertools;

#[derive(Constructor, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy)]
struct Point<T> {
    x: T,
    y: T,
}

type Galaxy = Point<usize>;

impl Display for Galaxy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day11.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn parse_input(input: &str) -> HashSet<Galaxy> {
    let mut galaxies = HashSet::new();
    for (row, line) in input.split("\n").filter(|line| !line.is_empty()).enumerate() {
        for (col, symbol) in line.trim().chars().enumerate() {
            if symbol == '#' {
                galaxies.insert(Point::new(col, row));
            }
        }
    }
    return galaxies;
}

fn part_one(input: &HashSet<Galaxy>) -> usize {
    expanded_galaxy_pair_distances(input, 2)
}

fn part_two(input: &HashSet<Galaxy>) -> usize {
    expanded_galaxy_pair_distances(input, 1000000)
}

fn expanded_galaxy_pair_distances(input: &HashSet<Galaxy>, expansion_factor: usize) -> usize {
    let max_x = input.iter().map(|p| p.x).max().unwrap();
    let max_y = input.iter().map(|p| p.y).max().unwrap();

    let mut doubled_x = (0..=max_x).collect::<HashSet<_>>();
    let mut doubled_y = (0..=max_y).collect::<HashSet<_>>();
    for g in input {
        doubled_x.remove(&g.x);
        doubled_y.remove(&g.y);
    }

    let mut sum = 0;
    for (a, b) in input.iter().tuple_combinations() {
        let dist = expanded_taxicab_distance(a, b, &doubled_x, &doubled_y, expansion_factor);
        sum += dist;
    }
    return sum;
}

fn expanded_taxicab_distance(a: &Galaxy, b: &Galaxy,
                             doubled_x: &HashSet<usize>, doubled_y: &HashSet<usize>,
                             expansion_factor: usize) -> usize {
    let max_x = max(a.x, b.x);
    let min_x = min(a.x, b.x);
    let max_y = max(a.y, b.y);
    let min_y = min(a.y, b.y);
    let expand_x = doubled_x.iter().filter(|x| min_x <= **x && **x <= max_x).count();
    let expand_y = doubled_y.iter().filter(|y| min_y <= **y && **y <= max_y).count();

    (max_x - min_x) + (max_y - min_y) + (expand_x + expand_y) * (expansion_factor - 1)
}