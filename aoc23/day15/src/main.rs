use std::fs;
use array_init::array_init;

#[derive(Clone)]
struct Lens {
    label: String,
    value: u8,
}

struct Snowbox(Vec<Lens>);

impl Snowbox {
    fn upsert(&mut self, label: String, value: u8) {
        let existing = self.0.iter().enumerate()
            .find_map(|(idx, lens)| if lens.label.eq(&label) { Some(idx) } else { None });
        if let Some(idx) = existing {
            *self.0.get_mut(idx).unwrap() = Lens { label, value };
        } else {
            self.0.push(Lens { label, value });
        }
    }

    fn delete(&mut self, label: String) {
        let existing = self.0.iter().enumerate()
            .find_map(|(idx, lens)| if lens.label.eq(&label) { Some(idx) } else { None });
        if let Some(idx) = existing {
            self.0.remove(idx);
        }
    }
}

struct Snowmap {
    boxes: [Snowbox; 256],
}

impl Snowmap {
    fn new() -> Self {
        Snowmap {
            boxes: array_init(|_| Snowbox(Vec::new())),
        }
    }

    fn upsert(&mut self, label: String, value: u8) {
        let hash = snow_hash(&label);
        self.boxes[hash as usize].upsert(label, value);
    }

    fn delete(&mut self, label: String) {
        let hash = snow_hash(&label);
        self.boxes[hash as usize].delete(label);
    }
}

enum Instruction {
    Set {
        label: String,
        value: u8,
    },
    Delete {
        label: String,
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day15.txt").unwrap();
    let input = parse_input(&input_s);
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &Vec<&str>) -> u64 {
    input.iter().map(|s| snow_hash(s) as u64).sum()
}

fn part_two(input: &Vec<&str>) -> u64 {
    let mut snowmap = Snowmap::new();
    for inst_str in input {
        let instruction = parse_instr(inst_str);
        match instruction {
            Instruction::Set { label, value } => {
                snowmap.upsert(label, value);
            },
            Instruction::Delete { label } => {
                snowmap.delete(label);
            }
        }
    }

    snowmap.boxes.iter().enumerate().map(|(box_idx, snowbox)| {
        snowbox.0.iter().enumerate().map(|(lens_idx, lens)| {
            ((box_idx + 1) * (lens_idx + 1) * (lens.value as usize)) as u64
        }).sum::<u64>()
    }).sum()
}

fn parse_instr(instr: &str) -> Instruction {
    if instr.ends_with("-") {
        let label = instr.strip_suffix("-").unwrap().to_owned();
        return Instruction::Delete { label };
    } else {
        let (label, value) = instr.split_once("=").unwrap();
        let label = label.to_owned();
        let value = value.parse::<u8>().unwrap();
        return Instruction::Set { label, value };
    }
}

fn snow_hash(string: &str) -> u8 {
    string.chars().fold(0u8, |acc, c| hash_step(acc, c))
}

fn hash_step(init: u8, next: char) -> u8 {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.

    let mut state = init as u32;
    assert!(next.is_ascii());
    assert!((next as u32) < 128);
    state += next as u32;
    state *= 17;
    state %= 256;
    return state as u8;
}

fn parse_input(input: &str) -> Vec<&str> {
    return input.trim().split(",").collect();
}