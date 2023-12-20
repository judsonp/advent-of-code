use anyhow::Result;
use clap::Parser;
use halfbrown::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{
    collections::{HashSet, VecDeque},
    fs,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Debug, Parser)]
#[command(about)]
struct Args {
    #[arg(short, long)]
    output_dotfile: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Voltage {
    Low,
    High,
}

#[derive(Debug, Clone)]
struct Pulse {
    voltage: Voltage,
    source: String,
    destination: String,
}

#[derive(Debug, Clone)]
enum Component {
    Broadcast {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        on: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        inputs: HashMap<String, Voltage>,
        outputs: Vec<String>,
    },
    Output {
        name: String,
        outputs: Vec<String>,
    },
}

impl Component {
    fn outputs(&self) -> &Vec<String> {
        match self {
            Component::Broadcast { outputs, .. } => outputs,
            Component::FlipFlop { outputs, .. } => outputs,
            Component::Conjunction { outputs, .. } => outputs,
            Component::Output { outputs, .. } => outputs,
        }
    }

    fn name(&self) -> &str {
        match self {
            Component::Broadcast { name, .. } => name,
            Component::FlipFlop { name, .. } => name,
            Component::Conjunction { name, .. } => name,
            Component::Output { name, .. } => name,
        }
    }

    fn pulse(&mut self, input: &Pulse) -> Vec<Pulse> {
        match self {
            Component::Broadcast { .. } => {
                return self
                    .outputs()
                    .iter()
                    .map(|output| Pulse {
                        voltage: input.voltage,
                        source: self.name().to_owned(),
                        destination: output.to_owned(),
                    })
                    .collect::<Vec<_>>();
            }
            Component::FlipFlop { on, .. } => {
                if input.voltage == Voltage::High {
                    return Vec::new();
                }
                *on = !(*on);
                let output_voltage = if *on { Voltage::High } else { Voltage::Low };
                return self
                    .outputs()
                    .iter()
                    .map(|output| Pulse {
                        voltage: output_voltage,
                        source: self.name().to_owned(),
                        destination: output.to_owned(),
                    })
                    .collect::<Vec<_>>();
            }
            Component::Conjunction { inputs, .. } => {
                *inputs.get_mut(&input.source).unwrap() = input.voltage;
                let output_voltage = if inputs.values().all(|v| *v == Voltage::High) {
                    Voltage::Low
                } else {
                    Voltage::High
                };
                return self
                    .outputs()
                    .iter()
                    .map(|output| Pulse {
                        voltage: output_voltage,
                        source: self.name().to_owned(),
                        destination: output.to_owned(),
                    })
                    .collect::<Vec<_>>();
            }
            Component::Output { .. } => Vec::new(),
        }
    }
}

type Input = HashMap<String, Component>;

fn main() {
    let args = Args::parse();

    let input_s = fs::read_to_string("inputs/day20.txt").unwrap();
    let (_, mut input) = parse_input(&input_s).unwrap();

    create_outputs(&mut input);
    wire_conjunctions(&mut input);

    if let Some(filename) = args.output_dotfile {
        write_dotfile(&filename, &input).expect("writing dotfile failed");
        println!("Wrote dotfile to {}", filename);
    }

    println!("Part one: {}", part_one(input));
}

fn write_dotfile(filename: &str, input: &Input) -> Result<()> {
    let path = Path::new(filename);
    let mut file = BufWriter::new(File::create(path)?);
    file.write_all("digraph Circuit {\n".as_bytes())?;
    for (src, component) in input {
        for dst in component.outputs() {
            file.write_all(format!("  {} -> {}\n", src, dst).as_bytes())?;
        }
    }
    file.write_all("}\n".as_bytes())?;
    Ok(())
}

fn part_one(mut components: Input) -> u64 {
    let mut sum_high = 0;
    let mut sum_low = 0;
    for _i in 0..1000 {
        let (high, low) = execute_counting(&mut components);
        sum_high += high;
        sum_low += low;
    }
    sum_high * sum_low
}

fn execute_counting(components: &mut Input) -> (u64, u64) {
    let mut queue = VecDeque::new();
    let mut count_high = 0;
    let mut count_low = 1;

    queue.push_back(Pulse {
        voltage: Voltage::Low,
        source: "button".to_owned(),
        destination: "broadcaster".to_owned(),
    });

    while let Some(pulse) = queue.pop_front() {
        let component = components
            .get_mut(&pulse.destination)
            .unwrap_or_else(|| panic!("Component {} not found", pulse.destination));
        let generated_pulses = component.pulse(&pulse);

        count_high += generated_pulses
            .iter()
            .filter(|p| p.voltage == Voltage::High)
            .count() as u64;
        count_low += generated_pulses
            .iter()
            .filter(|p| p.voltage == Voltage::Low)
            .count() as u64;

        queue.append(&mut VecDeque::from(generated_pulses));
    }

    (count_high, count_low)
}

fn create_outputs(input: &mut Input) {
    let outputs = input
        .iter()
        .flat_map(|(_, comp)| comp.outputs().iter())
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>();
    for output in outputs {
        if input.get(&output).is_none() {
            input.insert(
                output.clone(),
                Component::Output {
                    name: output.clone(),
                    outputs: Vec::new(),
                },
            );
        }
    }
}

fn wire_conjunctions(input: &mut Input) {
    let pairs = input
        .iter()
        .flat_map(|(src, comp)| comp.outputs().iter().map(move |dest| (src, dest)))
        .map(|(src, dest)| (src.to_owned(), dest.to_owned()))
        .collect::<Vec<_>>();

    for (src, dest) in pairs {
        if let Some(Component::Conjunction { inputs: state, .. }) = input.get_mut(&dest) {
            state.insert(src.clone(), Voltage::Low);
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(
        terminated(
            separated_list1(line_ending, parse_component),
            opt(line_ending),
        ),
        HashMap::from_iter,
    )(input)
}

fn parse_component(input: &str) -> IResult<&str, (String, Component)> {
    alt((parse_broadcast, parse_flipflop, parse_conjunction))(input)
}

fn parse_broadcast(input: &str) -> IResult<&str, (String, Component)> {
    map(
        preceded(tag("broadcaster -> "), parse_output_list),
        |outputs| {
            (
                "broadcaster".to_owned(),
                Component::Broadcast {
                    name: "broadcaster".to_owned(),
                    outputs,
                },
            )
        },
    )(input)
}

fn parse_flipflop(input: &str) -> IResult<&str, (String, Component)> {
    map(
        separated_pair(preceded(tag("%"), alpha1), tag(" -> "), parse_output_list),
        |(name, outputs)| {
            (
                name.to_owned(),
                Component::FlipFlop {
                    name: name.to_owned(),
                    on: false,
                    outputs,
                },
            )
        },
    )(input)
}

fn parse_conjunction(input: &str) -> IResult<&str, (String, Component)> {
    map(
        separated_pair(preceded(tag("&"), alpha1), tag(" -> "), parse_output_list),
        |(name, outputs)| {
            (
                name.to_owned(),
                Component::Conjunction {
                    name: name.to_owned(),
                    inputs: HashMap::new(),
                    outputs,
                },
            )
        },
    )(input)
}

fn parse_output_list(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), map(alpha1, |s: &str| s.to_owned()))(input)
}
