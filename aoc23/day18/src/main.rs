use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, hex_digit1, i64 as nom_i64, line_ending, space1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};
use std::{
    fmt::Display,
    fs,
    ops::{Add, Mul, Neg},
};

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

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            // Part 1
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            // Part 2
            '0' => Ok(Direction::Right),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '3' => Ok(Direction::Up),
            _ => Err(anyhow!("Invalid direction: {}", value)),
        }
    }
}

impl<T> Mul<T> for Direction
where
    T: Neg<Output = T> + Default,
{
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Direction::Up => Point::new(Default::default(), rhs),
            Direction::Down => Point::new(Default::default(), -rhs),
            Direction::Left => Point::new(-rhs, Default::default()),
            Direction::Right => Point::new(rhs, Default::default()),
        }
    }
}

struct Instruction {
    distance: i64,
    direction: Direction,
}

struct Input {
    instructions: Vec<Instruction>,
    alternate_instructions: Vec<Instruction>,
}

struct Polygon {
    points: Vec<Point<i64>>,
}

impl Polygon {
    fn area(&self) -> i64 {
        self.points
            .iter()
            .tuple_windows()
            .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
            .sum::<i64>()
            .abs()
            / 2
    }

    fn perimeter(&self) -> i64 {
        self.points
            .iter()
            .tuple_windows()
            .map(|(p1, p2)| {
                assert!(p1.x == p2.x || p1.y == p2.y);
                ((p1.x - p2.x) + (p1.y - p2.y)).abs()
            })
            .sum::<i64>()
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day18.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> i64 {
    let polygon = follow_instructions(&input.instructions);
    polygon.area() + (polygon.perimeter() / 2) + 1
}

fn part_two(input: &Input) -> i64 {
    let polygon = follow_instructions(&input.alternate_instructions);
    polygon.area() + (polygon.perimeter() / 2) + 1
}

fn follow_instructions(instructions: &[Instruction]) -> Polygon {
    let mut points = Vec::new();
    let mut cur = Point::new(0, 0);
    points.push(cur);
    for instruction in instructions {
        cur = cur + instruction.direction * instruction.distance;
        points.push(cur);
    }
    Polygon { points }
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        terminated(
            separated_list1(line_ending, parse_instruction),
            opt(line_ending),
        ),
        |instruction_pairs| {
            let (instructions, alternate_instructions) = instruction_pairs.into_iter().unzip();
            Input {
                instructions,
                alternate_instructions,
            }
        },
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
    map(
        tuple((
            terminated(alpha1, space1),
            terminated(nom_i64, space1),
            delimited(tag("(#"), hex_digit1, tag(")")),
        )),
        |(d, a, c)| (to_instruction(d, a), to_alternate_instruction(c)),
    )(input)
}

fn to_instruction(dir: &str, dist: i64) -> Instruction {
    Instruction {
        direction: dir.chars().next().unwrap().try_into().unwrap(),
        distance: dist,
    }
}

fn to_alternate_instruction(hex: &str) -> Instruction {
    let (dist, dir) = hex.split_at(5);
    let distance = i64::from_str_radix(dist, 16).unwrap();
    let direction = dir.chars().next().unwrap().try_into().unwrap();
    Instruction {
        direction,
        distance,
    }
}
