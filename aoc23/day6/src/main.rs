use std::cmp::{max, min};
use std::fs;
use std::iter::zip;
use nom::bytes::complete::tag;
use nom::character::complete::{i64 as nom64, line_ending, space0, space1};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded};

struct Race {
    duration: i64,
    distance_to_beat: i64,
}

fn main() {
    let input_s = fs::read_to_string("input.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
}

fn part_one(input: &Vec<Race>) -> i64 {
    input.iter().map(|r| ways_to_beat(r)).product()
}

fn ways_to_beat(race: &Race) -> i64 {
    let t: f64 = race.duration as f64;
    let d: f64 = race.distance_to_beat as f64;
    // find integers integers 0 <= b <= r where b * (r-b) > d
    // which is 1/2 (r - sqrt(r^2 - 4 d)) < b < 1/2 (sqrt(r^2 - 4 d) + r)
    let low = 0.5 * (t - (t.powi(2) - 4f64*d).sqrt());
    let high = 0.5 * (t + (t.powi(2) - 4f64*d).sqrt());
    let low = max(0, low.floor() as i64 + 1);
    let high = min(race.distance_to_beat, high.ceil() as i64 - 1);
    max(0, high - low + 1)
}

fn parse_input(text: &str) -> IResult<&str, Vec<Race>> {
    let (text, times) = delimited(pair(tag("Time:"), space0), separated_list0(space1, nom64), line_ending)(text)?;
    let (text, distances) = delimited(pair(tag("Distance:"), space0), separated_list0(space1, nom64), line_ending)(text)?;
    let races = zip(times, distances).map(|(t, d)| Race { duration: t, distance_to_beat: d }).collect();
    return Ok((text, races))
}