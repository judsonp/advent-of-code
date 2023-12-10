use std::collections::{HashMap, HashSet};
use std::fs;
use derive_more::Constructor;

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
    let input_s = fs::read_to_string("input.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Graph) -> u32 {
    let start = input.start;
    let mut steps: u32 = 0;
    let mut cur = start;

    steps += 1;
    cur = input.graph.get(&start).unwrap()[0];
    let mut last = start;

    while cur != start {
        let next = input.graph.get(&cur).unwrap().iter()
            .filter(|n| **n != last).next().unwrap();
        last = cur;
        cur = *next;
        steps += 1;
    }
    println!("Loop size: {}", steps);
    return steps / 2;
}

fn part_two(input: &Graph) -> u32 {
    let cage = find_loop(input);

    let max_x = input.graph.keys().map(|p| p.x).max().unwrap();
    let max_y = input.graph.keys().map(|p| p.y).max().unwrap();

    let mut inside_cage: u32 = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            if is_inside_cage(&Point::new(x, y), &cage, input) {
                inside_cage += 1;
            }
        }
    }

    return inside_cage;
}

fn is_inside_cage(point: &Point, cage: &HashSet<Point>, input: &Graph) -> bool {
    if cage.contains(point) {
        return false;
    }
    let cage_points_on_downright_diag =
        cage.iter().filter(|p| p.x > point.x && (p.x - point.x == p.y - point.y));
    let ignoring_bad_corners
        = cage_points_on_downright_diag.filter(|p| !is_tangent_diagonal(p, input.graph.get(p).unwrap()));
    let crossings = ignoring_bad_corners.count();
    return crossings % 2 == 1;
}

fn is_tangent_diagonal(p: &Point, adj: &[Point; 2]) -> bool {
    // 7 and L are ignorable
    // This isn't general, but it's the way we constructed 7 and L
    if adj[0] == Point::new(p.x, p.y - 1) && adj[1] == Point::new(p.x + 1, p.y) {
        return true;
    }
    if adj[0] == Point::new(p.x, p.y + 1) && adj[1] == Point::new(p.x - 1, p.y) {
        return true;
    }
    return false;
}

fn find_loop(input: &Graph) -> HashSet<Point> {
    let mut cage: HashSet<Point> = HashSet::new();

    let start = input.start;
    let mut cur = input.graph.get(&start).unwrap()[0];
    let mut last = start;

    cage.insert(start);

    while cur != start {
        cage.insert(cur);
        let next = input.graph.get(&cur).unwrap().iter()
            .filter(|n| **n != last).next().unwrap();
        last = cur;
        cur = *next;
    }

    return cage;
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
    let connected_to_start: [Point; 2] = graph.iter()
        .filter(|(loc, conns)| conns[0] == start || conns[1] == start)
        .map(|(loc, conns)| *loc)
        .collect::<Vec<_>>().try_into().unwrap();
    println!("Start: {:?}", start);
    println!("Connected to start: {:?}", connected_to_start);
    graph.insert(start, connected_to_start);

    return Graph { graph, start };
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