use grid::Grid;
use petgraph::algo::dijkstra;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct NodeId {
    x: u16,
    y: u16,
    steps: u8,
    direction: Direction,
}

impl NodeId {
    fn new(x: u16, y: u16, steps: u8, direction: Direction) -> Self {
        Self {
            x,
            y,
            steps,
            direction,
        }
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day17.txt").unwrap();
    let input = parse_input(&input_s);
    let graph = build_graph(&input);
    println!("Part one: {}", part_one(&input, &graph));
}

fn part_one(input: &Grid<u8>, graph: &GraphMap<NodeId, u32, Directed>) -> u32 {
    let start = NodeId::new(0, 0, 0, Direction::None);
    let costs = dijkstra(graph, start, None, |(_, _, &weight)| weight);

    all_dirsteps_iter(input.cols() - 1, input.rows() - 1)
        .filter_map(|id| costs.get(&id))
        .cloned()
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Grid<u8> {
    let input = input.trim();
    let height = input.split("\n").count();
    let width = input.split("\n").next().unwrap().len();
    let mut grid = Grid::init(height, width, 0);
    for (y, line) in input.split("\n").enumerate() {
        for (x, value) in line.chars().enumerate() {
            *grid.get_mut(y, x).unwrap() = value.to_digit(10).unwrap() as u8;
        }
    }
    return grid;
}

fn all_dirsteps_iter(x: usize, y: usize) -> impl Iterator<Item = NodeId> {
    (1..=3).flat_map(move |steps| {
        DIRECTIONS
            .iter()
            .map(move |&dir| NodeId::new(x as u16, y as u16, steps, dir))
    })
}

fn all_nodes_iter(cols: usize, rows: usize) -> impl Iterator<Item = NodeId> {
    (0..cols).flat_map(move |x| (0..rows).flat_map(move |y| all_dirsteps_iter(x, y)))
}

fn directed_steps(steps: u8, prev_dir: Direction, new_dir: Direction) -> u8 {
    if prev_dir == new_dir {
        steps + 1
    } else {
        1
    }
}

fn add_direction(id: NodeId, dir: Direction, rows: usize, cols: usize) -> Option<NodeId> {
    let pos = match dir {
        Direction::Up if id.y == 0 => None,
        Direction::Up => Some((id.x, id.y - 1)),
        Direction::Down if id.y == (rows as u16) - 1 => None,
        Direction::Down => Some((id.x, id.y + 1)),
        Direction::Left if id.x == 0 => None,
        Direction::Left => Some((id.x - 1, id.y)),
        Direction::Right if id.x == (cols as u16) - 1 => None,
        Direction::Right => Some((id.x + 1, id.y)),
        Direction::None => unreachable!(),
    };
    pos.map(|(x, y)| NodeId::new(x, y, directed_steps(id.steps, id.direction, dir), dir))
}

fn opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::None => Direction::None,
    }
}

fn node_neighbors(id: NodeId, rows: usize, cols: usize) -> impl Iterator<Item = NodeId> {
    DIRECTIONS
        .iter()
        .filter_map(move |&dir| add_direction(id, dir, rows, cols))
        // can't take more than 3 steps in the same direction
        .filter(|neighbor| neighbor.steps <= 3)
        // can't turn around
        .filter(move |neighbor| neighbor.direction != opposite_direction(id.direction))
}

fn build_graph(input: &Grid<u8>) -> GraphMap<NodeId, u32, Directed> {
    let mut graph = GraphMap::new();
    let start = NodeId::new(0, 0, 0, Direction::None);

    for id in all_nodes_iter(input.cols(), input.rows()) {
        graph.add_node(id);
    }
    // starting node, which has weird properties
    graph.add_node(start);

    for id in all_nodes_iter(input.cols(), input.rows()) {
        for neighbor in node_neighbors(id, input.rows(), input.cols()) {
            graph.add_edge(
                id,
                neighbor,
                *input.get(neighbor.y as usize, neighbor.x as usize).unwrap() as u32,
            );
        }
    }
    for neighbor in node_neighbors(start, input.rows(), input.cols()) {
        graph.add_edge(
            start,
            neighbor,
            *input.get(neighbor.y as usize, neighbor.x as usize).unwrap() as u32,
        );
    }

    return graph;
}
