use grid::Grid;
use itertools::Itertools;
use std::fs;

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

#[derive(Clone)]
struct Rocks {
    rocks: Grid<RockState>,
}

impl Rocks {
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
}

fn part_one(input: &Rocks) -> usize {
    let mut grid = input.clone();
    tilt_north(&mut grid);
    rocks_value(&grid)
}

fn tilt_north(rocks: &mut Rocks) {
    for col in 0..rocks.rocks.cols() {
        let mut target = 0;
        for row in 0..rocks.rocks.rows() {
            let val = *rocks.rocks.get(row, col).unwrap();

            if row == target {
                if val != RockState::Empty {
                    target += 1;
                }
                continue;
            }

            match val {
                RockState::Empty => {}
                RockState::Square => {
                    target = row + 1;
                }
                RockState::Smooth => {
                    *rocks.rocks.get_mut(target, col).unwrap() = val;
                    *rocks.rocks.get_mut(row, col).unwrap() = RockState::Empty;
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
