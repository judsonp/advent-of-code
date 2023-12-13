use std::fmt::{Display, Formatter};
use std::fs;
use std::vec::IntoIter;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1, u64 as nom_u64};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use rangemap::RangeSet;
use rayon::prelude::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl Display for SpringState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SpringState::Operational => '.',
            SpringState::Damaged => '#',
            SpringState::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
struct Spring {
    states: Vec<SpringState>,
    groups: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    springs: Vec<Spring>,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day12.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &Input) -> usize {
    input.springs.par_iter()
        .map(|spring| combinations(spring)).sum()
}

fn combinations(spring: &Spring) -> usize {
    spring.states.iter()
        .map(|s| spring_options(s))
        .multi_cartesian_product()
        .filter(|option| count_groups(option).eq(&spring.groups))
        .count()
}

fn spring_options(s: &SpringState) -> IntoIter<SpringState> {
    match s {
        SpringState::Operational => vec![SpringState::Operational].into_iter(),
        SpringState::Damaged => vec![SpringState::Damaged].into_iter(),
        SpringState::Unknown => vec![SpringState::Operational, SpringState::Damaged].into_iter(),
    }
}

fn count_groups(state: &Vec<SpringState>) -> Vec<usize> {
    let ranges = state.iter().enumerate()
        .filter(|(_, s)| **s != SpringState::Operational)
        .map(|(i, _)| i)
        .map(|i| i..i + 1)
        .collect::<RangeSet<_>>();
    ranges.into_iter().map(|range| {
        range.end - range.start
    }).collect::<Vec<_>>()
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(separated_list1(line_ending, parse_spring),
        |v| Input { springs: v })(input)
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    map(separated_pair(parse_spring_state, space1, parse_group_sizes),
        |(state, sizes)| Spring {
            states: state,
            groups: sizes,
        })(input)
}

fn parse_spring_state(input: &str) -> IResult<&str, Vec<SpringState>> {
    many1(map(one_of(".#?"), |c| match c {
        '.' => SpringState::Operational,
        '#' => SpringState::Damaged,
        '?' => SpringState::Unknown,
        _ => panic!("Impossible spring state char: {}", c),
    }))(input)
}

fn parse_group_sizes(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map(nom_u64, |n| n as usize))(input)
}
