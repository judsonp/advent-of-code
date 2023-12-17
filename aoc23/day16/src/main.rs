use grid::Grid;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "<",
                Direction::Right => ">",
                Direction::Up => "^",
                Direction::Down => "v",
            }
        )
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum OpticalPart {
    Empty,
    SplitterUpDown,
    SplitterLeftRight,
    // reflects light coming from the left to down and vice versa; same as up right
    MirrorDown,
    // reflects light coming from the left to up and vice versa; same as down right
    MirrorUp,
}

impl From<char> for OpticalPart {
    fn from(value: char) -> Self {
        match value {
            '|' => OpticalPart::SplitterUpDown,
            '-' => OpticalPart::SplitterLeftRight,
            '/' => OpticalPart::MirrorUp,
            '\\' => OpticalPart::MirrorDown,
            '.' => OpticalPart::Empty,
            _ => panic!("Invalid part: {}", value),
        }
    }
}

impl Display for OpticalPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OpticalPart::Empty => '.',
                OpticalPart::MirrorDown => '\\',
                OpticalPart::MirrorUp => '/',
                OpticalPart::SplitterLeftRight => '-',
                OpticalPart::SplitterUpDown => '|',
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Beam {
    loc: Point,
    dir: Direction,
}

impl Beam {
    fn new(loc: Point, dir: Direction) -> Self {
        Self { loc, dir }
    }
}

struct Opgrid {
    grid: Grid<OpticalPart>,
}

impl Opgrid {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.grid
            .iter_rows()
            .map(|row| row.map(|item| item.to_string()).join(""))
            .join("\n")
    }
}

struct Illumination {
    grid: Grid<bool>,
}

impl Illumination {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        self.grid
            .iter_rows()
            .map(|row| row.map(|x| if *x { "#" } else { "." }).join(""))
            .join("\n")
    }

    fn new(ops: &Opgrid) -> Self {
        Illumination {
            grid: Grid::init(ops.grid.rows(), ops.grid.cols(), false),
        }
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day16.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(ops: &Opgrid) -> usize {
    return illumination_score(ops, Beam::new(Point::new(0, 0), Direction::Right));
}

fn part_two(ops: &Opgrid) -> usize {
    let rights = (0..ops.grid.rows()).map(|r| Beam::new(Point::new(0, r), Direction::Right));
    let downs = (0..ops.grid.cols()).map(|c| Beam::new(Point::new(c, 0), Direction::Down));
    let lefts = (0..ops.grid.rows())
        .map(|r| Beam::new(Point::new(ops.grid.cols() - 1, r), Direction::Left));
    let ups =
        (0..ops.grid.cols()).map(|c| Beam::new(Point::new(c, ops.grid.rows() - 1), Direction::Up));
    let beams = rights.chain(downs).chain(lefts).chain(ups).collect_vec();

    return beams
        .par_iter()
        .map(|beam| illumination_score(ops, *beam))
        .max()
        .unwrap();
}

fn illumination_score(ops: &Opgrid, beam: Beam) -> usize {
    let mut illum = Illumination::new(&ops);
    illuminate(&ops, &mut illum, beam);
    return illum.grid.iter().map(|x| if *x { 1 } else { 0 }).sum();
}

fn illuminate(ops: &Opgrid, illum: &mut Illumination, beam: Beam) {
    propagate_beam(ops, illum, beam, &mut HashSet::new());
}

fn propagate_beam(ops: &Opgrid, illum: &mut Illumination, beam: Beam, visited: &mut HashSet<Beam>) {
    // cycle check
    if visited.contains(&beam) {
        return;
    }

    // illuminate
    *illum.grid.get_mut(beam.loc.y, beam.loc.x).unwrap() = true;
    visited.insert(beam);

    let propagated_beams = get_next_beams(ops, &beam);
    for maybe_next_beam in propagated_beams.into_iter() {
        if let Some(next_beam) = maybe_next_beam {
            propagate_beam(ops, illum, next_beam, visited);
        }
    }
}

fn bounded_propagate_loc(ops: &Opgrid, loc: &Point, dir: &Direction) -> Option<Beam> {
    match dir {
        Direction::Left if loc.x == 0 => None,
        Direction::Left => Some(Beam::new(Point::new(loc.x - 1, loc.y), *dir)),
        Direction::Right if loc.x == ops.grid.cols() - 1 => None,
        Direction::Right => Some(Beam::new(Point::new(loc.x + 1, loc.y), *dir)),
        Direction::Up if loc.y == 0 => None,
        Direction::Up => Some(Beam::new(Point::new(loc.x, loc.y - 1), *dir)),
        Direction::Down if loc.y == ops.grid.rows() - 1 => None,
        Direction::Down => Some(Beam::new(Point::new(loc.x, loc.y + 1), *dir)),
    }
}

fn get_next_beams(ops: &Opgrid, beam: &Beam) -> [Option<Beam>; 2] {
    let cell = ops.grid.get(beam.loc.y, beam.loc.x).unwrap();

    match cell {
        OpticalPart::Empty => [bounded_propagate_loc(ops, &beam.loc, &beam.dir), None],
        OpticalPart::SplitterUpDown => match beam.dir {
            Direction::Left | Direction::Right => [
                bounded_propagate_loc(ops, &beam.loc, &Direction::Up),
                bounded_propagate_loc(ops, &beam.loc, &Direction::Down),
            ],
            _ => [bounded_propagate_loc(ops, &beam.loc, &beam.dir), None],
        },
        OpticalPart::SplitterLeftRight => match beam.dir {
            Direction::Up | Direction::Down => [
                bounded_propagate_loc(ops, &beam.loc, &Direction::Left),
                bounded_propagate_loc(ops, &beam.loc, &Direction::Right),
            ],
            _ => [bounded_propagate_loc(ops, &beam.loc, &beam.dir), None],
        },
        OpticalPart::MirrorDown => {
            let dir = match beam.dir {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
            };
            [bounded_propagate_loc(ops, &beam.loc, &dir), None]
        }
        OpticalPart::MirrorUp => {
            let dir = match beam.dir {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
            };
            [bounded_propagate_loc(ops, &beam.loc, &dir), None]
        }
    }
}

fn parse_input(input: &str) -> Opgrid {
    let input = input.trim();
    let height = input.split("\n").count();
    let width = input.split("\n").next().unwrap().len();
    let mut grid = Grid::init(height, width, OpticalPart::Empty);
    for (y, line) in input.split("\n").enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            *grid.get_mut(y, x).unwrap() = symbol.into();
        }
    }
    return Opgrid { grid };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "\\/.\n|-.\n";
        let parsed = parse_input(input);
        let grid = &parsed.grid;
        assert_eq!(grid.cols(), 3);
        assert_eq!(grid.rows(), 2);
        assert_eq!(grid.get(0, 0).unwrap(), &OpticalPart::MirrorDown);
        assert_eq!(grid.get(0, 1).unwrap(), &OpticalPart::MirrorUp);
        assert_eq!(grid.get(0, 2).unwrap(), &OpticalPart::Empty);
        assert_eq!(grid.get(1, 0).unwrap(), &OpticalPart::SplitterUpDown);
        assert_eq!(grid.get(1, 1).unwrap(), &OpticalPart::SplitterLeftRight);
        assert_eq!(grid.get(1, 2).unwrap(), &OpticalPart::Empty);
    }

    #[test]
    fn example() {
        let input_s = fs::read_to_string("../examples/day16.txt").unwrap();
        let input = parse_input(&input_s);
        assert_eq!(part_one(&input), 46);
    }

    #[test]
    fn tsplit() {
        let input = "..|..\n.....\n..-..";
        let parsed = super::parse_input(input);
        assert_eq!(part_one(&parsed), 9);
    }

    #[test]
    fn mirrors() {
        let input = "...\\...\n.......\n-......\n.......\n\\../...";
        let parsed = super::parse_input(input);
        assert_eq!(part_one(&parsed), 18);
    }

    #[test]
    fn loop_with_initial_dirchange() {
        let input = "|....-\n......\n......\n-....|";
        let parsed = super::parse_input(input);
        assert_eq!(part_one(&parsed), 16);
    }

    #[test]
    fn multivisitor() {
        let input = "......|...\\..\\...\n..../........|...\n....\\.-.../......\n......|....../...\n.................";
        let parsed = super::parse_input(input);
        assert_eq!(part_one(&parsed), 41);
    }

    #[test]
    fn misc_testcase() {
        let input = "\\........-.........\\................................|...\n......-/.............|-.../.....|...........././..\\.....\n-.........................|.....\\...................|.\\.\n.......-........../.......\\.........|..../........-.-|..\n";
        let parsed = super::parse_input(input);
        assert_eq!(part_one(&parsed), 89);
    }

    #[test]
    fn example_part2() {
        let input_s = fs::read_to_string("../examples/day16.txt").unwrap();
        let input = parse_input(&input_s);
        assert_eq!(part_two(&input), 51);
    }
}
