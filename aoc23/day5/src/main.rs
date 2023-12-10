use std::cmp::{max, min};
use std::fs;
use std::ops::Range;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{i64 as nom64, multispace1, space1, space0, line_ending, multispace0};
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, tuple};
use rangemap::{RangeMap, RangeSet};

#[derive(Debug)]
struct ElfMap {
    data: RangeMap<i64, i64>,
}

impl From<Vec<(i64, i64, i64)>> for ElfMap {
    fn from(input: Vec<(i64, i64, i64)>) -> Self {
        let mut result = RangeMap::new();
        for (dest_start, src_start, range) in input {
            result.insert(src_start..src_start+range, dest_start - src_start);
        }
        return ElfMap { data: result }
    }
}

impl ElfMap {
    fn map_value(&self, input: i64) -> i64 {
        input + self.data.get(&input).unwrap_or(&0)
    }

    fn map_values(&self, values: &Vec<i64>) -> Vec<i64> {
        values.iter().map(|v| self.map_value(*v)).collect()
    }

    fn map_ranges(&self, ranges: &RangeSet<i64>) -> RangeSet<i64> {
        let mut sharded_ranges: RangeSet<i64> = RangeSet::new();
        for src_range in ranges.iter() {
            for (dst_range, offset) in self.data.overlapping(src_range) {
                let intersecting_range = range_intersect(src_range, dst_range);
                let final_range = intersecting_range.start+offset..intersecting_range.end+offset;
                sharded_ranges.insert(final_range);
            }
            for dst_range in self.data.gaps(src_range) {
                sharded_ranges.insert(range_intersect(src_range, &dst_range));
            }
        }
        return sharded_ranges
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    seed_ranges: RangeSet<i64>,
    maps: Vec<ElfMap>,
}

fn main() {
    let input_s = fs::read_to_string("inputs/day5.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Input) -> i64 {
    input.maps.iter()
        .fold(input.seeds.clone(),
        |s, m| m.map_values(&s))
        .iter()
        .min()
        .unwrap().clone()
}

fn part_two(input: &Input) -> i64 {
    input.maps.iter()
        .fold(input.seed_ranges.clone(),
              |r, m| m.map_ranges(&r))
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn range_intersect(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(tuple((seedlist, many1(elfmap))),
        |(seeds, maps)|
            Input { seed_ranges: seed_input_to_ranges(&seeds), seeds, maps })
        (input)
}

fn seed_input_to_ranges(seeds: &Vec<i64>) -> RangeSet<i64> {
    let mut ret = RangeSet::new();
    for slice in seeds.chunks_exact(2) {
        ret.insert(slice[0]..slice[0]+slice[1]);
    }
    return ret;
}

fn seedlist(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(tag("seeds: "), separated_list1(space1, nom64), multispace1)(input)
}

fn elfmap(input: &str) -> IResult<&str, ElfMap> {
    map(delimited(tuple((is_not(":"), tag(":"), multispace0)), elfmap_data, multispace1),
        |x| x.into())(input)
}

fn elfmap_data(input: &str) -> IResult<&str, Vec<(i64, i64, i64)>> {
    separated_list1(line_ending,
                    tuple((delimited(space0, nom64, space0),
                           delimited(space0, nom64, space0),
                           delimited(space0, nom64, space0))))(input)
}