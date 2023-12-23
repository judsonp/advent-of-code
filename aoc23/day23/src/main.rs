use anyhow::anyhow;
use anyhow::Error;
use grid::Grid;
use petgraph::algo::all_simple_paths;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use std::fmt::Display;
use std::fs;
use std::ops::Add;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    Wall,
    Open,
    Down,
    Up,
    Left,
    Right,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Wall),
            '.' => Ok(Self::Open),
            'v' => Ok(Self::Down),
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(anyhow!("unrecognized character")),
        }
    }
}

struct Scene(Grid<Tile>);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
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

type Location = Point<i16>;

const DELTAS: [Location; 4] = [
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: 0, y: 1 },
];

fn bounded_neighbors<'a>(
    location: &'a Location,
    bounds: &'a Location,
    deltas: &'a [Location],
) -> impl Iterator<Item = Location> + 'a {
    deltas
        .iter()
        .map(|delta| *location + *delta)
        .filter(|n| n.x > 0 && n.y > 0 && n.x < bounds.x && n.y < bounds.y)
}

fn neighbors<'a>(
    location: &'a Location,
    bounds: &'a Location,
    tile: Tile,
) -> impl Iterator<Item = Location> + 'a {
    match tile {
        Tile::Wall => bounded_neighbors(location, bounds, &[]),
        Tile::Open => bounded_neighbors(location, bounds, &DELTAS),
        Tile::Down => bounded_neighbors(location, bounds, &DELTAS[3..=3]),
        Tile::Up => bounded_neighbors(location, bounds, &DELTAS[2..=2]),
        Tile::Left => bounded_neighbors(location, bounds, &DELTAS[0..=0]),
        Tile::Right => bounded_neighbors(location, bounds, &DELTAS[1..=1]),
    }
}

struct SceneGraph {
    graph: GraphMap<Location, (), Directed>,
    start: Location,
    end: Location,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day23.txt").unwrap();
    let input = parse_input(&input_s).unwrap();
    let graph = make_graph(&input).unwrap();
    println!("Part one: {}", part_one(&graph).unwrap());
}

fn part_one(graph: &SceneGraph) -> Result<usize, Error> {
    let paths = all_simple_paths::<Vec<_>, _>(&graph.graph, graph.start, graph.end, 1, None);
    paths
        .map(|path| path.len() - 1)
        .max()
        .ok_or(anyhow!("no paths found"))
}

fn make_graph(input: &Scene) -> Result<SceneGraph, Error> {
    let bounds = Location::new(input.0.cols() as i16, input.0.rows() as i16);
    let mut graph = GraphMap::new();
    let mut start = None;
    let mut end = None;

    for row in 0..input.0.rows() {
        for col in 0..input.0.cols() {
            let tile = *input.0.get(row, col).ok_or(anyhow!("missing point"))?;
            let loc = Location::new(col as i16, row as i16);
            if tile == Tile::Wall {
                continue;
            }
            if row == 0 {
                start = Some(loc);
            }
            if row == input.0.rows() - 1 {
                end = Some(loc);
            }
            for neighbor in neighbors(&loc, &bounds, tile) {
                let neighbor_tile = *input
                    .0
                    .get(neighbor.y, neighbor.x)
                    .ok_or(anyhow!("missing point"))?;
                if neighbor_tile != Tile::Wall {
                    graph.add_edge(loc, neighbor, ());
                }
            }
        }
    }

    Ok(SceneGraph {
        graph,
        start: start.ok_or(anyhow!("didn't find start"))?,
        end: end.ok_or(anyhow!("didn't find end"))?,
    })
}

fn parse_input(input: &str) -> Result<Scene, Error> {
    let rows = input.trim().split('\n').count();
    let cols = input
        .trim()
        .split('\n')
        .next()
        .ok_or(anyhow!("empty first line"))?
        .trim()
        .len();
    let mut scene = Grid::init(rows, cols, Tile::Wall);
    for (row, line) in input.trim().split('\n').enumerate() {
        let line = line.trim();
        for (col, symbol) in line.chars().enumerate() {
            *scene.get_mut(row, col).ok_or(anyhow!("invalid coords"))? = Tile::try_from(symbol)?;
        }
    }

    Ok(Scene(scene))
}
