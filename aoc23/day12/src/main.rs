use std::fmt::{Display, Formatter};
use std::{fs, iter};
use std::cmp::min;
use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space1, u64 as nom_u64};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
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
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> usize {
    input.springs.par_iter()
        .map(|spring| combinations(&spring.states, &spring.groups, &mut HashMap::new()))
        .sum()
}

fn part_two(input: &Input) -> usize {
    input.springs
        .par_iter()
        .map(|spring| unfold(spring))
        .map(|spring| combinations(&spring.states, &spring.groups, &mut HashMap::new()))
        .sum()
}

fn combinations(states: &[SpringState], groups: &[usize], cache: &mut HashMap<(usize, usize),usize>) -> usize {
    if groups.is_empty() {
        return if states.iter().any(|&s| s == SpringState::Damaged) {
            0
        } else {
            1
        }
    }

    // groups is not empty, so this is not a valid state
    if states.is_empty() {
        return 0;
    }

    // try to locate the first group starting at starting position X
    // all positions < X are SpringState::Operational
    // positions [X, X + group_size) are SpringState::Damaged
    // unless X + group_size is off the end of the slice, it must be SpringState::Operational
    let mut total_combinations = 0;

    let group_size = groups[0];
    let remaining_groups = &groups[1..];

    if states.len() < group_size {
        return 0;
    }

    for tent_start_pos in 0..=(states.len()-group_size) {
        // If this is true, we cannot consider all of the springs in the proposed range
        // to be Damaged, because at least one of them is Operational.
        let damaged_range_invalid = states[tent_start_pos..tent_start_pos+group_size]
            .iter().any(|&s| s == SpringState::Operational);
        // If this is true, then the proposed range cannot be this group of Damaged springs,
        // because the subsequent spring is also Damaged (so there is not the necessary terminator).
        let operational_terminator_impossible = (states.len() > tent_start_pos + group_size) &&
            (states[tent_start_pos + group_size] == SpringState::Damaged);
        // If this is true, this is the last possible tentative placement, since after this point,
        // one of the springs to the left of the tentative placement will be damaged.
        let last_possible_placement = states[tent_start_pos] == SpringState::Damaged;

        if !damaged_range_invalid && !operational_terminator_impossible {
            // Possible range placement
            let remaining_state = &states[min(states.len(), tent_start_pos+group_size+1)..];
            total_combinations += combinations_cached(remaining_state, remaining_groups, cache);
        }

        if last_possible_placement {
            return total_combinations;
        }
    }

    return total_combinations;
}

fn combinations_cached(states: &[SpringState], groups: &[usize],
                       cache: &mut HashMap<(usize,usize),usize>) -> usize {
    let key = (states.len(), groups.len());
    if let Some(result) = cache.get(&key) {
        return *result;
    } else {
        let result = combinations(states, groups, cache);
        cache.insert(key, result);
        return result;
    }
}

fn unfold(spring: &Spring) -> Spring {
    let groups = iter::repeat(&spring.groups).take(5).flatten().cloned().collect_vec();
    let states = iter::repeat(&spring.states).take(5)
        .intersperse(&vec![SpringState::Unknown])
        .flatten().cloned().collect_vec();
    Spring { states, groups }
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
