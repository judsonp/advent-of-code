use derive_more::Constructor;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Constructor, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Constructor)]
struct Graph {
    graph: HashMap<Point, [Point; 2]>,
    start: Point,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day10.txt").unwrap();
    let input = parse_input(&input_s);
    let cage = find_loop(&input);
    println!("Part one: {}", part_one(&cage));
    println!("Part two: {}", part_two(&cage));
}

fn part_one(cage: &Vec<Point>) -> u32 {
    cage.len() as u32 / 2
}

fn part_two(cage: &Vec<Point>) -> u32 {
    // Pick's theorem: area = interior + (boundary / 2) - 1
    // interior = area - (boundary / 2) + 1
    let area = polygon_area(cage);
    let peri = cage.len() as u32;
    area - (peri / 2) + 1
}

fn find_loop(input: &Graph) -> Vec<Point> {
    let mut cage = Vec::new();

    let start = input.start;
    let mut cur = input.graph.get(&start).unwrap()[0];
    let mut last = start;

    cage.push(start);

    while cur != start {
        cage.push(cur);
        let next = input
            .graph
            .get(&cur)
            .unwrap()
            .iter()
            .find(|n| **n != last)
            .unwrap();
        last = cur;
        cur = *next;
    }

    cage.push(cur);

    cage
}

fn polygon_area(points: &Vec<Point>) -> u32 {
    points
        .iter()
        .tuple_windows()
        .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
        .sum::<i32>()
        .abs() as u32
        / 2
}

fn parse_input(input: &str) -> Graph {
    let mut graph: HashMap<Point, [Point; 2]> = HashMap::new();
    let mut start: Option<Point> = None;
    for (row, line) in input.lines().enumerate() {
        for (col, symbol) in line.trim().chars().enumerate() {
            if let Some(conn) = connections(row as i32, col as i32, symbol) {
                graph.insert(Point::new(col as i32, row as i32), conn);
            }
            if symbol == 'S' {
                start = Some(Point::new(col as i32, row as i32));
            }
        }
    }

    let start = start.unwrap();
    let connected_to_start: [Point; 2] = graph
        .iter()
        .filter(|(_, conns)| conns[0] == start || conns[1] == start)
        .map(|(loc, _)| *loc)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    graph.insert(start, connected_to_start);

    Graph { graph, start }
}

fn connections(row: i32, col: i32, symbol: char) -> Option<[Point; 2]> {
    match symbol {
        '|' => Some([Point::new(col, row + 1), Point::new(col, row - 1)]),
        '-' => Some([Point::new(col + 1, row), Point::new(col - 1, row)]),
        'L' => Some([Point::new(col, row - 1), Point::new(col + 1, row)]),
        'J' => Some([Point::new(col, row - 1), Point::new(col - 1, row)]),
        '7' => Some([Point::new(col, row + 1), Point::new(col - 1, row)]),
        'F' => Some([Point::new(col, row + 1), Point::new(col + 1, row)]),
        '.' => None,
        'S' => None,
        _ => panic!("Bad input: {}", symbol),
    }
}
