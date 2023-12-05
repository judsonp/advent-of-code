use std::cmp::{max, min};
use std::fs;
use std::ops::Range;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{i64 as nom64, multispace1, space1, space0, line_ending, multispace0};
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list1;
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
    fn map(&self, input: i64) -> i64 {
        input + self.data.get(&input).unwrap_or(&0)
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    seed_ranges: RangeSet<i64>,
    seed_to_soil: ElfMap,
    soil_to_fert: ElfMap,
    fert_to_water: ElfMap,
    water_to_light: ElfMap,
    light_to_temp: ElfMap,
    temp_to_humid: ElfMap,
    humid_to_loc: ElfMap,
}

impl Input {
    fn seed_to_loc(&self, seed: i64) -> i64 {
        let soil = self.seed_to_soil.map(seed);
        let fert = self.soil_to_fert.map(soil);
        let water = self.fert_to_water.map(fert);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let humid = self.temp_to_humid.map(temp);
        let loc = self.humid_to_loc.map(humid);
        return loc;
    }
}

fn main() {
    let input_s = fs::read_to_string("input.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Input) -> i64 {
    input.seeds.iter()
        .map(|seed| input.seed_to_loc(*seed))
        .min().unwrap()
}

fn part_two(input: &Input) -> i64 {
    let soils = map_ranges(&input.seed_ranges, &input.seed_to_soil.data);
    let ferts = map_ranges(&soils, &input.soil_to_fert.data);
    let waters = map_ranges(&ferts, &input.fert_to_water.data);
    let lights = map_ranges(&waters, &input.water_to_light.data);
    let temps = map_ranges(&lights, &input.light_to_temp.data);
    let humids = map_ranges(&temps, &input.temp_to_humid.data);
    let locs = map_ranges(&humids, &input.humid_to_loc.data);
    locs.iter().map(|r| r.start).min().unwrap()
}

fn range_intersect(a: &Range<i64>, b: &Range<i64>) -> Range<i64> {
    max(a.start, b.start)..min(a.end, b.end)
}

fn map_ranges(ranges: &RangeSet<i64>, mapping: &RangeMap<i64, i64>) -> RangeSet<i64> {
    let mut sharded_ranges: RangeSet<i64> = RangeSet::new();
    for src_range in ranges.iter() {
        for (dst_range, offset) in mapping.overlapping(src_range) {
            let intersecting_range = range_intersect(src_range, dst_range);
            let final_range = intersecting_range.start+offset..intersecting_range.end+offset;
            sharded_ranges.insert(final_range);
        }
        for dst_range in mapping.gaps(src_range) {
            sharded_ranges.insert(range_intersect(src_range, &dst_range));
        }
    }
    return sharded_ranges
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    map(tuple((seedlist, elfmap, elfmap, elfmap, elfmap, elfmap, elfmap, elfmap)),
        |(seeds, seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humid, humid_to_loc)|
            Input { seed_ranges: seed_input_to_ranges(&seeds), seeds, seed_to_soil, soil_to_fert, fert_to_water, water_to_light, light_to_temp, temp_to_humid, humid_to_loc })
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