use grid::Grid;
use itertools::Itertools;
use std::fs;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum RockState {
    Empty,
    Square,
    Smooth,
}

impl From<char> for RockState {
    fn from(value: char) -> Self {
        match value {
            '.' => RockState::Empty,
            '#' => RockState::Square,
            'O' => RockState::Smooth,
            _ => panic!("Invalid rock state: {}", value),
        }
    }
}

impl From<RockState> for char {
    fn from(value: RockState) -> Self {
        match value {
            RockState::Empty => '.',
            RockState::Square => '#',
            RockState::Smooth => 'O',
        }
    }
}

#[derive(Clone, PartialEq)]
struct Rocks {
    rocks: Grid<RockState>,
}

impl Rocks {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.rocks
            .iter_rows()
            .map(|row| row.map(|&item| -> char { item.into() }).join(""))
            .join("\n")
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day14.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Rocks) -> usize {
    let mut grid = input.clone();
    tilt(&mut grid, Direction::North);
    rocks_value(&grid)
}

fn part_two(input: &Rocks) -> usize {
    const TOTAL_CYCLES: usize = 1000000000;

    let mut grid = input.clone();

    let mut past_states = Vec::new();
    past_states.push(grid.clone());

    let mut iter = 1;
    loop {
        spin_cycle(&mut grid);
        let cycle = past_states
            .iter()
            .enumerate()
            .filter(|(_, r)| grid.eq(r))
            .map(|(idx, _)| idx)
            .next();
        if let Some(start) = cycle {
            let cycle_length = iter - start;
            let cycle_iters = TOTAL_CYCLES - start;
            let offset = cycle_iters % cycle_length;
            let answer_idx = start + offset;
            return rocks_value(&past_states[answer_idx]);
        } else {
            past_states.push(grid.clone());
            iter += 1;
        }
    }
}

fn spin_cycle(rocks: &mut Rocks) {
    tilt(rocks, Direction::North);
    tilt(rocks, Direction::West);
    tilt(rocks, Direction::South);
    tilt(rocks, Direction::East);
}

fn view_get<T>(grid: &Grid<T>, outer: usize, inner: usize, direction: Direction) -> &T {
    let (col, row) = match direction {
        Direction::North => (outer, inner),
        Direction::South => (outer, grid.rows() - inner - 1),
        Direction::East => (grid.cols() - inner - 1, outer),
        Direction::West => (inner, outer),
    };
    grid.get(row, col).unwrap()
}

fn view_get_mut<T>(grid: &mut Grid<T>, outer: usize, inner: usize, direction: Direction) -> &mut T {
    let (col, row) = match direction {
        Direction::North => (outer, inner),
        Direction::South => (outer, grid.rows() - inner - 1),
        Direction::East => (grid.cols() - inner - 1, outer),
        Direction::West => (inner, outer),
    };
    grid.get_mut(row, col).unwrap()
}

fn tilt(rocks: &mut Rocks, dir: Direction) {
    let (out_max, in_max) = match dir {
        Direction::North | Direction::South => (rocks.rocks.cols(), rocks.rocks.rows()),
        Direction::West | Direction::East => (rocks.rocks.rows(), rocks.rocks.cols()),
    };

    for outer in 0..out_max {
        let mut target = 0;
        for inner in 0..in_max {
            let val = *view_get(&rocks.rocks, outer, inner, dir);

            if inner == target {
                if val != RockState::Empty {
                    target += 1;
                }
                continue;
            }

            match val {
                RockState::Empty => {}
                RockState::Square => {
                    target = inner + 1;
                }
                RockState::Smooth => {
                    *view_get_mut(&mut rocks.rocks, outer, target, dir) = val;
                    *view_get_mut(&mut rocks.rocks, outer, inner, dir) = RockState::Empty;
                    target += 1;
                }
            }
        }
    }
}

fn rocks_value(rocks: &Rocks) -> usize {
    rocks
        .rocks
        .iter_rows()
        .enumerate()
        .map(|(row_idx, row)| {
            let row_score = rocks.rocks.rows() - row_idx;
            row.filter(|val| **val == RockState::Smooth).count() * row_score
        })
        .sum()
}

fn parse_input(input: &str) -> Rocks {
    let rows = input.trim().split("\n").count();
    let cols = input.trim().split("\n").next().unwrap().trim().len();
    let mut rocks = Grid::init(rows, cols, RockState::Empty);
    for (row, line) in input.trim().split("\n").enumerate() {
        let line = line.trim();
        for (col, symbol) in line.chars().enumerate() {
            *rocks.get_mut(row, col).unwrap() = RockState::from(symbol);
        }
    }

    Rocks { rocks }
}
