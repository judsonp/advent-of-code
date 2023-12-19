use enum_map::{Enum, EnumMap};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1, one_of, u16 as parse_u16},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    Accept,
    Reject,
    Workflow { workflow: String },
}

#[derive(Debug, Enum, Clone, Copy, PartialEq, Eq)]
enum Property {
    ExtremeCoolness,
    Musicality,
    Aerodynamicity,
    Shininess,
}

impl From<char> for Property {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::ExtremeCoolness,
            'm' => Self::Musicality,
            'a' => Self::Aerodynamicity,
            's' => Self::Shininess,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparator {
    Less,
    Greater,
}

impl From<char> for Comparator {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Less,
            '>' => Self::Greater,
            _ => unreachable!(),
        }
    }
}

impl Comparator {
    fn matches<T>(&self, lhs: &T, rhs: &T) -> bool
    where
        T: PartialOrd,
    {
        match self {
            Comparator::Less => lhs < rhs,
            Comparator::Greater => lhs > rhs,
        }
    }
}

#[derive(Debug)]
struct Condition {
    property: Property,
    comparator: Comparator,
    quantity: u16,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<(Option<Condition>, Target)>,
}

#[derive(Debug)]
struct Part {
    properties: EnumMap<Property, u16>,
}

impl Part {
    fn score(&self) -> u64 {
        self.properties.values().map(|&v| v as u64).sum::<u64>()
    }
}

#[derive(Debug)]
struct Input {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day19.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Input) -> u64 {
    input
        .parts
        .iter()
        .filter(|part| destination(&input.workflows, part) == &Target::Accept)
        .map(|part| part.score())
        .sum::<u64>()
}

fn destination<'a>(workflows: &'a HashMap<String, Workflow>, part: &Part) -> &'a Target {
    apply_workflow(workflows, "in", part)
}

fn apply_workflow<'a>(
    workflows: &'a HashMap<String, Workflow>,
    workflow_name: &str,
    part: &Part,
) -> &'a Target {
    let workflow = workflows.get(workflow_name).unwrap();
    for (maybe_condition, target) in &workflow.rules {
        if let Some(condition) = maybe_condition {
            if matches_condition(condition, part) {
                return match target {
                    Target::Accept | Target::Reject => target,
                    Target::Workflow { workflow } => apply_workflow(workflows, workflow, part),
                };
            }
        } else {
            return match target {
                Target::Accept | Target::Reject => target,
                Target::Workflow { workflow } => apply_workflow(workflows, workflow, part),
            };
        }
    }
    unreachable!();
}

fn matches_condition(condition: &Condition, part: &Part) -> bool {
    condition
        .comparator
        .matches(&part.properties[condition.property], &condition.quantity)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        terminated(
            separated_pair(parse_workflows, multispace1, parse_parts),
            line_ending,
        ),
        |(rules, parts)| Input {
            workflows: rules,
            parts,
        },
    )(input)
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<String, Workflow>> {
    map(separated_list1(line_ending, parse_workflow), |rules| {
        HashMap::from_iter(rules)
    })(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    // hxf{s<2974:dd,s<3412:fp,m<3807:jt,rmt}
    map(
        tuple((
            alpha1,
            tag("{"),
            separated_list1(tag(","), parse_rule),
            tag("}"),
        )),
        |(name, _, parts, _)| (name.to_owned(), Workflow { rules: parts }),
    )(input)
}

fn parse_rule(input: &str) -> IResult<&str, (Option<Condition>, Target)> {
    alt((parse_normal_rule, parse_default_rule))(input)
}

fn parse_normal_rule(input: &str) -> IResult<&str, (Option<Condition>, Target)> {
    map(
        tuple((
            parse_property,
            parse_comparator,
            parse_u16,
            tag(":"),
            parse_target,
        )),
        |(property, comparator, quantity, _, target)| {
            (
                Some(Condition {
                    property,
                    comparator,
                    quantity,
                }),
                target,
            )
        },
    )(input)
}

fn parse_property(input: &str) -> IResult<&str, Property> {
    map(one_of("xmas"), |p| p.into())(input)
}

fn parse_comparator(input: &str) -> IResult<&str, Comparator> {
    map(one_of("<>"), |c| c.into())(input)
}

fn parse_default_rule(input: &str) -> IResult<&str, (Option<Condition>, Target)> {
    map(parse_target, |target| (None, target))(input)
}

fn parse_target(input: &str) -> IResult<&str, Target> {
    alt((
        map(tag("A"), |_| Target::Accept),
        map(tag("R"), |_| Target::Reject),
        map(alpha1, |workflow: &str| Target::Workflow {
            workflow: workflow.to_owned(),
        }),
    ))(input)
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(line_ending, parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    // {x=653,m=2123,a=2908,s=577}
    map(
        delimited(
            tag("{"),
            separated_list1(
                tag(","),
                separated_pair(parse_property, tag("="), parse_u16),
            ),
            tag("}"),
        ),
        |props: Vec<(Property, u16)>| Part {
            properties: EnumMap::from_iter(props),
        },
    )(input)
}
