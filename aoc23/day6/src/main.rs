use std::cmp::{max, min};
use std::fs;
use std::iter::zip;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{i64 as nom64, line_ending, space0, space1};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair};

struct Race {
    duration: i64,
    distance_to_beat: i64,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day6.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    let (_, input2) = parse_input_2(&input_s).unwrap();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input2));
}

fn part_one(input: &Vec<Race>) -> i64 {
    input.iter().map(|r| ways_to_beat(r)).product()
}

fn part_two(race: &Race) -> i64 {
    ways_to_beat(race)
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

fn parse_input_2(text: &str) -> IResult<&str, Race> {
    let (text, timestring) = delimited(pair(tag("Time:"), space0), is_not("\n"), line_ending)(text)?;
    let (text, diststring) = delimited(pair(tag("Distance:"), space0), is_not("\n"), line_ending)(text)?;

    let time = str::replace(timestring, " ", "").parse::<i64>().unwrap();
    let dist = str::replace(diststring, " ", "").parse::<i64>().unwrap();

    return Ok((text, Race { duration: time, distance_to_beat: dist }))
}
