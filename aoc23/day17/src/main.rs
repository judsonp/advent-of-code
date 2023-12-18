use grid::Grid;
use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for usize {
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
    x: usize,
    y: usize,
    steps: usize,
    direction: Direction,
}

impl NodeId {
    fn new(x: usize, y: usize, steps: usize, direction: Direction) -> Self {
        Self {
            x,
            y,
            steps,
            direction,
        }
    }

    fn to_index(self, _max_x: usize, max_y: usize, max_steps: usize) -> usize {
        // note: max x/y is exclusive, max steps is inclusive
        // note: steps = 0 is not a supported state
        assert!(self.steps != 0);
        assert!(self.direction != Direction::None);
        let mut id = self.x;
        id = (id * max_y) + self.y;
        id = (id * max_steps) + (self.steps - 1);
        id = (id * 4) + <Direction as Into<usize>>::into(self.direction);
        id
    }

    fn from_index(id: usize, _max_x: usize, max_y: usize, max_steps: usize) -> Self {
        let mut rem = id;
        let dir_no = rem % 4;
        rem /= 4;
        let steps = (rem % max_steps) + 1;
        rem /= max_steps;
        let y = rem % max_y;
        rem /= max_y;
        let x = rem;
        Self {
            x,
            y,
            steps,
            direction: dir_no.into(),
        }
    }
}

struct LimitedPriorityQueue<T> {
    size: usize,
    base_priority: usize,
    max_priority_skew: usize,
    queues: Vec<Vec<T>>,
}

impl<T> LimitedPriorityQueue<T> {
    fn new(max_priority_skew: usize) -> Self {
        Self {
            size: 0,
            base_priority: 0,
            max_priority_skew,
            queues: (0..max_priority_skew).map(|_| Vec::new()).collect(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, id: T, priority: usize) {
        let mut adjusted_priority = priority - self.base_priority;

        if adjusted_priority >= self.max_priority_skew {
            let shift = adjusted_priority - self.max_priority_skew + 1;
            for i in 0..shift {
                assert!(self.queues[i].is_empty());
            }
            self.queues.rotate_left(shift);
            adjusted_priority -= shift;
            self.base_priority += shift;
        }

        self.queues[adjusted_priority].push(id);
        self.size += 1;
    }

    fn pop(&mut self) -> (T, usize) {
        assert!(self.size > 0);
        for i in 0..self.queues.len() {
            if !self.queues[i].is_empty() {
                let id = self.queues[i].pop().unwrap();
                let priority = self.base_priority + i;
                self.size -= 1;
                return (id, priority);
            }
        }
        unreachable!();
    }
}

struct Problem<'a> {
    grid: &'a Grid<usize>,
    min_steps: usize,
    max_steps: usize,
}

impl Problem<'_> {
    fn node_to_index(&self, node: &NodeId) -> usize {
        node.to_index(self.grid.cols(), self.grid.rows(), self.max_steps)
    }

    fn index_to_node(&self, id: usize) -> NodeId {
        NodeId::from_index(id, self.grid.cols(), self.grid.rows(), self.max_steps)
    }

    fn max_index(&self) -> usize {
        self.grid.cols() * self.grid.rows() * self.max_steps * 4
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day17.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Grid<usize>) -> usize {
    let problem = Problem {
        grid: input,
        min_steps: 1,
        max_steps: 3,
    };
    crucible_walk(&problem)
}

fn part_two(input: &Grid<usize>) -> usize {
    let problem = Problem {
        grid: input,
        min_steps: 4,
        max_steps: 10,
    };
    crucible_walk(&problem)
}

fn crucible_walk(problem: &Problem) -> usize {
    let start = NodeId::new(0, 0, 0, Direction::None);
    let start_sentinel = usize::MAX;

    let mut costs = vec![0; problem.max_index()];
    let mut queue = LimitedPriorityQueue::new(10);

    queue.push(start_sentinel, 0);

    while !queue.is_empty() {
        let (cur_id, cur_cost) = queue.pop();

        // skip already-marked nodes
        if cur_id != start_sentinel && costs[cur_id] != 0 {
            continue;
        }

        // mark this node
        if cur_id != start_sentinel {
            costs[cur_id] = cur_cost;
        }

        let cur = if cur_id == start_sentinel {
            start
        } else {
            problem.index_to_node(cur_id)
        };

        if cur.x == problem.grid.cols() - 1
            && cur.y == problem.grid.rows() - 1
            && cur.steps >= problem.min_steps
        {
            return cur_cost;
        }

        let neighbors = node_neighbors(
            cur,
            problem.grid.rows(),
            problem.grid.cols(),
            problem.min_steps,
            problem.max_steps,
        );

        for neighbor in neighbors {
            let neighbor_id = problem.node_to_index(&neighbor);
            let neighbor_cost = cur_cost + *problem.grid.get(neighbor.y, neighbor.x).unwrap();
            queue.push(neighbor_id, neighbor_cost);
        }
    }

    unreachable!();
}

fn parse_input(input: &str) -> Grid<usize> {
    let input = input.trim();
    let height = input.split('\n').count();
    let width = input.split('\n').next().unwrap().len();
    let mut grid = Grid::init(height, width, 0);
    for (y, line) in input.split('\n').enumerate() {
        for (x, value) in line.chars().enumerate() {
            *grid.get_mut(y, x).unwrap() = value.to_digit(10).unwrap() as usize;
        }
    }
    grid
}

fn directed_steps(steps: usize, prev_dir: Direction, new_dir: Direction) -> usize {
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
        Direction::Down if id.y == rows - 1 => None,
        Direction::Down => Some((id.x, id.y + 1)),
        Direction::Left if id.x == 0 => None,
        Direction::Left => Some((id.x - 1, id.y)),
        Direction::Right if id.x == cols - 1 => None,
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
    min_steps: usize,
    max_steps: usize,
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
