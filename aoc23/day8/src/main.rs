use std::collections::HashMap;
use std::fs;
use num::integer::lcm;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, multispace0, one_of, space0};
use nom::combinator::{complete, map};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated};

type NodeLabel = String;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: NodeLabel,
    right: NodeLabel,
}

#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    graph: HashMap<NodeLabel, Node>,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day8.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> usize {
    let mut current_node = "AAA";
    let mut count: usize = 0;
    while !current_node.eq("ZZZ") {
        let direction = &input.directions[count % input.directions.len()];
        current_node = advance_node(input, current_node, direction);
        count += 1;
    }
    return count;
}

fn advance_node<'a>(input: &'a Input, current: &str, direction: &Direction) -> &'a str {
    let node = input.graph.get(current).unwrap();
    return match direction {
        Direction::Left => &node.left,
        Direction::Right => &node.right,
    };
}

fn part_two(input: &Input) -> usize {
    let start_nodes: Vec<&str> = input.graph.keys()
        .filter(|label| label.ends_with("A"))
        .map(|s| -> &str { s }).collect();
    let mut path_lengths: Vec<usize> = Vec::new();
    for start_node in start_nodes {
        let path_len = check_and_get_path_length(start_node, input);
        path_lengths.push(path_len.expect("path does not have expected properties"));
    }
    return path_lengths.iter().fold(1usize, |a, b| lcm(a, *b));
}

fn check_and_get_path_length(start_node: &str, input: &Input) -> Option<usize> {
    let (initial_end_node, count) = path_to_end(start_node, input);
    let (loop_end_node, loop_count) = path_to_end(start_node, input);
    // These are convenient properties of our input.
    if loop_end_node != initial_end_node {
        return None;
    }
    if count != loop_count {
        return None;
    }
    return Some(count);
}

fn path_to_end<'a>(start: &'a str, input: &'a Input) -> (&'a str, usize) {
    let mut count: usize = 0;
    let mut current_node = start;
    loop {
        let direction = &input.directions[count % input.directions.len()];
        current_node = advance_node(input, current_node, direction);
        count += 1;
        if current_node.ends_with("Z") {
            return (current_node, count);
        }
    }
}

fn parse_input(text: &str) -> IResult<&str, Input> {
    let (_, (directions, nodes)) = complete(separated_pair(directions, multispace0, nodelist))(text)?;
    let mut graph: HashMap<NodeLabel, Node> = HashMap::new();

    for (label, node) in nodes {
        graph.insert(label, node);
    }

    return Ok(("", Input { directions, graph }));
}

fn directions(text: &str) -> IResult<&str, Vec<Direction>> {
    terminated(many1(map(one_of("LR"), |c| {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid input: {}", c),
        }
    })), multispace0)(text)
}

fn nodelist(text: &str) -> IResult<&str, Vec<(NodeLabel, Node)>> {
    terminated(separated_list1(line_ending, node), multispace0)(text)
}

fn node(text: &str) -> IResult<&str, (NodeLabel, Node)> {
    separated_pair(nodelabel, delimited(space0, tag("="), space0), nodedata)(text)
}

fn nodedata(text: &str) -> IResult<&str, Node> {
    map(delimited(tag("("), separated_pair(nodelabel, delimited(space0, tag(","), space0), nodelabel), tag(")")),
        |(left, right)| Node { left, right })(text)
}

fn nodelabel(text: &str) -> IResult<&str, NodeLabel> {
    map(alpha1, |s: &str| -> NodeLabel { s.to_string() })(text)
}