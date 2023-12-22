use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use derive_more::Constructor;
use nom::{
    bytes::complete::tag,
    character::complete::i64 as parse_i64,
    character::complete::line_ending,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use range_ext::intersect::Intersect;

#[derive(Constructor, Clone, Copy, PartialEq, Eq, Debug)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Constructor, Clone, Copy, Debug)]
struct Brick(Point3d, Point3d);

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        let this_xrange = min(self.0.x, self.1.x)..=max(self.0.x, self.1.x);
        let this_yrange = min(self.0.y, self.1.y)..=max(self.0.y, self.1.y);
        let other_xrange = min(other.0.x, other.1.x)..=max(other.0.x, other.1.x);
        let other_yrange = min(other.0.y, other.1.y)..=max(other.0.y, other.1.y);

        this_xrange.intersect(&other_xrange).is_any()
            && this_yrange.intersect(&other_yrange).is_any()
    }
}

type Input = Vec<Brick>;

fn main() {
    let input_s = fs::read_to_string("inputs/day22.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part one: {}", part_one(input));
}

fn part_one(mut bricks: Input) -> usize {
    order_bricks(&mut bricks);
    let on_top_of = drop_bricks(&mut bricks);
    assert!(on_top_of.len() == bricks.len());

    let mut ok_to_disintegrate = vec![true; bricks.len()];
    for (_, deps) in on_top_of {
        if deps.len() == 1 {
            ok_to_disintegrate[deps[0]] = false;
        }
    }

    ok_to_disintegrate.iter().filter(|ok| **ok).count()
}

fn order_bricks(bricks: &mut Input) {
    bricks.sort_unstable_by_key(|brick| max(brick.0.z, brick.1.z));
}

fn drop_bricks(bricks: &mut Input) -> HashMap<usize, Vec<usize>> {
    let mut on_top_of = HashMap::new();

    for i in 0..bricks.len() {
        let mut brick = bricks[i];
        let below = bricks[0..i]
            .iter()
            .enumerate()
            .filter(|(_, b)| max(b.0.z, b.1.z) < min(brick.0.z, brick.1.z))
            .filter(|(_, b)| brick.overlaps(b));

        let max_z = below
            .clone()
            .map(|(_, b)| max(b.0.z, b.1.z))
            .max()
            .unwrap_or(-1);

        let brick_on_top_of = below
            .filter(|(_, b)| max(b.0.z, b.1.z) == max_z)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        on_top_of.insert(i, brick_on_top_of);

        let drop = min(brick.0.z, brick.1.z) - max_z - 1;
        assert!(drop >= 0);
        brick.0.z -= drop;
        brick.1.z -= drop;
        bricks[i] = brick;
    }

    on_top_of
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    terminated(separated_list1(line_ending, parse_brick), opt(line_ending))(input)
}

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    map(
        separated_pair(parse_point, tag("~"), parse_point),
        |(a, b)| Brick::new(a, b),
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point3d> {
    map(
        tuple((parse_i64, tag(","), parse_i64, tag(","), parse_i64)),
        |(x, _, y, _, z)| Point3d::new(x, y, z),
    )(input)
}
