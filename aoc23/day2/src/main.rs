use std::fs;
use nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{u32 as nom32, space1, line_ending};
use nom::combinator::{all_consuming, map, opt};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::{pair, separated_pair, terminated, tuple};
use enum_map::{enum_map, Enum, EnumMap};

#[derive(Debug, PartialEq, Enum)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Draw {
    draw: EnumMap<Color, u32>,
}

impl Draw {
    fn contains(&self, other: &Draw) -> bool {
        self.draw.iter()
            .all(|(c, n)| other.draw[c] <= *n)
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

#[derive(Debug, PartialEq)]
struct Input {
    games: Vec<Game>,
}

fn main() {
    let input_s = fs::read_to_string("input.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("{}", part_one(input));
}

fn part_one(input: Input) -> u32 {
    let ref_contents = Draw { draw: enum_map! {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    } };
    input.games.iter()
        .filter(|g| g.draws.iter().all(|d| ref_contents.contains(d)))
        .map(|g| g.id)
        .sum()
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue)))(input)
}

fn draw_item(input: &str) -> IResult<&str, (Color, u32)> {
    map(separated_pair(nom32, space1, color), |(n, c)| (c, n))(input)
}

fn draw(input: &str) -> IResult<&str, Draw> {
    map(separated_list0(tag(", "), draw_item),
        |d| Draw { draw: EnumMap::from_iter(d.into_iter()) })(input)
}

fn draws(input: &str) -> IResult<&str, Vec<Draw>> {
    separated_list0(tag("; "), draw)(input)
}

fn header(input: &str) -> IResult<&str, u32> {
    map(tuple((tag("Game"), space1, nom32, tag(":"), space1)), |r| r.2)(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(pair(header, draws), |(n, d)| Game { id: n, draws: d })(input)
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    all_consuming(map(terminated(separated_list0(line_ending, game), opt(line_ending)),
                      |g| Input { games: g }))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(color("green"),
                   Ok(("", Color::Green)))
    }

    #[test]
    fn parse_draw() {
        assert_eq!(draw("3 green"),
                   Ok(("", Draw { draw: enum_map!{ Color::Green => 3, _ => 0 } })))
    }

    #[test]
    fn parse_draw_multi() {
        assert_eq!(draw("3 green, 1 blue"),
                   Ok(("", Draw { draw: enum_map!{ Color::Green => 3, Color::Blue => 1, _ => 0 } })))
    }

    #[test]
    fn parse_draws() {
        assert_eq!(draws("3 green; 1 red"),
                   Ok(("", vec![Draw { draw: enum_map!{Color::Green => 3, _ => 0}},
                                Draw { draw: enum_map!{Color::Red => 1, _ => 0}}])))
    }

    #[test]
    fn parse_draws_multi() {
        assert_eq!(draws("3 green, 1 blue; 1 red, 2 green"),
                   Ok(("", vec![Draw { draw: enum_map!{Color::Green => 3, Color::Blue => 1, _ => 0}},
                                Draw { draw: enum_map!{Color::Red => 1, Color::Green => 2, _ => 0}}])))
    }

    #[test]
    fn parse_header() {
        assert_eq!(header("Game 4: "), Ok(("", 4)))
    }

    #[test]
    fn parse_game() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let g = game(s);
        assert!(g.is_ok());
        assert_eq!(g.unwrap().0, "");
    }

    #[test]
    fn parse_example() {
        let s = fs::read_to_string("example.txt").unwrap();
        let g = parse_input(&s);
        assert!(g.is_ok());
        assert_eq!(g.unwrap().0, "");
    }

    #[test]
    fn example() {
        let s = fs::read_to_string("example.txt").unwrap();
        let (_, i) = parse_input(&s).unwrap();
        assert_eq!(part_one(i), 8);
    }
}