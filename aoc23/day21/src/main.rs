use std::{collections::HashSet, fmt::Display, fs, ops::Add};

use grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Plot {
    Garden,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Display for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Add<Point<T>> for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

type Location = Point<i64>;

struct Input {
    grid: Grid<Plot>,
    bounds: Location,
    start: Location,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day21.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &Input) -> usize {
    let mut locations = HashSet::new();

    locations.insert(input.start);

    for _ in 0..64 {
        let mut new_locs = HashSet::new();
        for loc in locations.drain() {
            new_locs.extend(
                bounded_neighbors(&loc, &input.bounds)
                    .filter(|loc| *input.grid.get(loc.y, loc.x).unwrap() != Plot::Rock),
            );
        }
        locations.extend(new_locs.drain());
    }

    locations.len()
}

fn bounded_neighbors<'a>(
    location: &'a Location,
    bounds: &'a Location,
) -> impl Iterator<Item = Location> + 'a {
    const DELTAS: [Location; 4] = [
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
    ];

    DELTAS
        .iter()
        .map(|delta| *location + *delta)
        .filter(|n| n.x > 0 && n.y > 0 && n.x < bounds.x && n.y < bounds.y)
}

fn parse_input(input: &str) -> Input {
    let input = input.trim();
    let height = input.split('\n').count();
    let width = input.split('\n').next().unwrap().len();
    let mut grid = Grid::init(height, width, Plot::Garden);
    let mut start = None;
    for (y, line) in input.split('\n').enumerate() {
        for (x, value) in line.chars().enumerate() {
            if value == '#' {
                *grid.get_mut(y, x).unwrap() = Plot::Rock;
            }
            if value == 'S' {
                start = Some(Point::new(x as i64, y as i64));
            }
        }
    }

    Input {
        bounds: Point::new(grid.cols() as i64, grid.rows() as i64),
        grid,
        start: start.unwrap(),
    }
}
