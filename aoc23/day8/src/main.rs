use std::collections::HashMap;
use std::fs;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, line_ending, multispace0, one_of, space0};
use nom::combinator::{complete, map};
use nom::{InputIter, IResult};
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
    let input_s = fs::read_to_string("input.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &Input) -> usize {
    let mut cur_nodelabel = "AAA";
    let mut count: usize = 0;
    while !cur_nodelabel.eq("ZZZ") {
        let node = input.graph.get(cur_nodelabel).unwrap();
        let direction = &input.directions[count % input.directions.len()];
        cur_nodelabel = match direction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
        count += 1;
    }
    return count;
}

fn parse_input(text: &str) -> IResult<&str, Input> {
    let (_, (directions, nodes)) = complete(separated_pair(directions, multispace0, nodelist))(text)?;
    let mut graph: HashMap<NodeLabel, Node> = HashMap::new();

    for (label, node) in nodes {
        graph.insert(label, node);
    }

    return Ok(("", Input{directions, graph}));
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