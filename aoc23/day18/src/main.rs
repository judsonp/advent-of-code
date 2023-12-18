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
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
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
    color: String,
}

struct Input {
    instructions: Vec<Instruction>,
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
    println!("Part 1: {}", part_one(&input));
}

fn part_one(input: &Input) -> i64 {
    let polygon = follow_instructions(input);
    polygon.area() + (polygon.perimeter() / 2) + 1
}

fn follow_instructions(input: &Input) -> Polygon {
    let mut points = Vec::new();
    let mut cur = Point::new(0, 0);
    points.push(cur);
    for instruction in &input.instructions {
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
        |instructions| Input { instructions },
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            terminated(alpha1, space1),
            terminated(nom_i64, space1),
            delimited(tag("(#"), hex_digit1, tag(")")),
        )),
        |(d, a, c)| to_instruction(d, a, c),
    )(input)
}

fn to_instruction(dir: &str, dist: i64, col: &str) -> Instruction {
    Instruction {
        direction: dir.chars().next().unwrap().try_into().unwrap(),
        distance: dist,
        color: col.to_owned(),
    }
}
