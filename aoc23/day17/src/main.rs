use grid::Grid;
use std::collections::BinaryHeap;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for u32 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
            Direction::None => unreachable!(),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

#[derive(PartialEq, Eq, Clone, Copy)]
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

    fn to_index(self, _max_x: u16, max_y: u16, max_steps: u8) -> u32 {
        // note: max x/y is exclusive, max steps is inclusive
        // note: steps = 0 is not a supported state
        assert!(self.steps != 0);
        assert!(self.direction != Direction::None);
        let mut id = self.x as u32;
        id = (id * max_y as u32) + self.y as u32;
        id = (id * max_steps as u32) + (self.steps as u32 - 1);
        id = (id * 4) + <Direction as Into<u32>>::into(self.direction);
        id
    }

    fn from_index(id: u32, _max_x: u16, max_y: u16, max_steps: u8) -> Self {
        let mut rem = id;
        let dir_no = rem % 4;
        rem /= 4;
        let steps = (rem % max_steps as u32) + 1;
        rem /= max_steps as u32;
        let y = rem % max_y as u32;
        rem /= max_y as u32;
        let x = rem;
        Self {
            x: x as u16,
            y: y as u16,
            steps: steps as u8,
            direction: dir_no.into(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct PathState {
    cost: u32,
    id: u32,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .cmp(&other.cost)
            .reverse()
            .then(self.id.cmp(&other.id))
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Problem<'a> {
    grid: &'a Grid<u8>,
    min_steps: u8,
    max_steps: u8,
}

impl Problem<'_> {
    fn node_to_index(&self, node: &NodeId) -> u32 {
        node.to_index(
            self.grid.cols() as u16,
            self.grid.rows() as u16,
            self.max_steps,
        )
    }

    fn index_to_node(&self, id: u32) -> NodeId {
        NodeId::from_index(
            id,
            self.grid.cols() as u16,
            self.grid.rows() as u16,
            self.max_steps,
        )
    }

    fn max_index(&self) -> u32 {
        (self.grid.cols() * self.grid.rows() * (self.max_steps as usize) * 4) as u32
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day17.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Grid<u8>) -> u32 {
    let problem = Problem {
        grid: input,
        min_steps: 1,
        max_steps: 3,
    };
    crucible_walk(&problem)
}

fn part_two(input: &Grid<u8>) -> u32 {
    let problem = Problem {
        grid: input,
        min_steps: 4,
        max_steps: 10,
    };
    crucible_walk(&problem)
}

fn crucible_walk(problem: &Problem) -> u32 {
    let start = NodeId::new(0, 0, 0, Direction::None);
    let start_sentinel = u32::MAX;

    let mut costs = vec![0; problem.max_index() as usize];
    let mut queue = BinaryHeap::new();

    queue.push(PathState {
        cost: 0,
        id: start_sentinel,
    });

    while !queue.is_empty() {
        let cur_state: PathState = queue.pop().unwrap();

        // skip already-marked nodes
        if cur_state.id != start_sentinel && costs[cur_state.id as usize] != 0 {
            continue;
        }

        // mark this node
        if cur_state.id != start_sentinel {
            costs[cur_state.id as usize] = cur_state.cost;
        }

        let cur = if cur_state.id == start_sentinel {
            start
        } else {
            problem.index_to_node(cur_state.id)
        };

        let neighbors = node_neighbors(
            cur,
            problem.grid.rows(),
            problem.grid.cols(),
            problem.min_steps,
            problem.max_steps,
        );

        for neighbor in neighbors {
            let neighbor_id = problem.node_to_index(&neighbor);
            let neighbor_cost = cur_state.cost
                + *problem
                    .grid
                    .get(neighbor.y as usize, neighbor.x as usize)
                    .unwrap() as u32;
            let neighbor_state = PathState {
                cost: neighbor_cost,
                id: neighbor_id,
            };
            queue.push(neighbor_state);
        }
    }

    all_dirsteps_iter(
        problem.grid.cols() - 1,
        problem.grid.rows() - 1,
        problem.min_steps,
        problem.max_steps,
    )
    .map(|id| problem.node_to_index(&id))
    .map(|idx| costs[idx as usize])
    .filter(|&cost| cost != 0)
    .min()
    .unwrap()
}

fn parse_input(input: &str) -> Grid<u8> {
    let input = input.trim();
    let height = input.split('\n').count();
    let width = input.split('\n').next().unwrap().len();
    let mut grid = Grid::init(height, width, 0);
    for (y, line) in input.split('\n').enumerate() {
        for (x, value) in line.chars().enumerate() {
            *grid.get_mut(y, x).unwrap() = value.to_digit(10).unwrap() as u8;
        }
    }
    grid
}

fn all_dirsteps_iter(
    x: usize,
    y: usize,
    min_steps: u8,
    max_steps: u8,
) -> impl Iterator<Item = NodeId> {
    (min_steps..=max_steps).flat_map(move |steps| {
        DIRECTIONS
            .iter()
            .map(move |&dir| NodeId::new(x as u16, y as u16, steps, dir))
    })
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

fn node_neighbors(
    id: NodeId,
    rows: usize,
    cols: usize,
    min_steps: u8,
    max_steps: u8,
) -> impl Iterator<Item = NodeId> {
    DIRECTIONS
        .iter()
        .filter_map(move |&dir| add_direction(id, dir, rows, cols))
        // can't turn around
        .filter(move |neighbor| neighbor.direction != opposite_direction(id.direction))
        // must move at least min_steps in the same direction before turning
        .filter(move |neighbor| {
            id.steps >= min_steps
                || neighbor.direction == id.direction
                || id.direction == Direction::None
        })
        // can't take more than max_steps steps in the same direction
        .filter(move |neighbor| neighbor.steps <= max_steps)
}
