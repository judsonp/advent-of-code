use std::fs;
use nom::bytes::complete::tag;
use nom::character::complete::{u32 as nom32, line_ending, space0, space1};
use nom::combinator::{all_consuming, map, opt};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, terminated, tuple};

struct Card {
    winning: Vec<u32>,
    have: Vec<u32>,
}

struct Input {
    cards: Vec<Card>,
}

fn main() {
    let input_s = fs::read_to_string("input.txt").unwrap();
    let (_, input) = parse_input(&input_s).unwrap();
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &Input) -> u32 {
    input.cards.iter()
        .map(|c| card_score(c))
        .sum()
}

fn part_two(input: &Input) -> u32 {
    let mut total_cards: u32 = 0;
    let mut copies = vec![1; input.cards.len()];

    for (i, card) in input.cards.iter().enumerate() {
        let matches = card.have.iter().filter(|n| card.winning.contains(n)).count();
        let this_copies = copies[i];
        for v in &mut copies[i+1..i+1+matches] {
            *v += this_copies;
        }
        total_cards += this_copies;
    }

    return total_cards;
}

fn card_score(card: &Card) -> u32 {
    let count = card.have.iter().filter(|n| card.winning.contains(n)).count();
    if count > 0 { 1 << (count - 1) } else { 0 }
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    all_consuming(map(terminated(separated_list1(line_ending, card), opt(line_ending)),
                      |c| Input { cards: c }))(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    map(tuple((header, numberlist, delimited(space0, tag("|"), space0), numberlist)),
        |(_, winning, _, have)| Card { winning, have })(input)
}

fn header(input: &str) -> IResult<&str, u32> {
    delimited(terminated(tag("Card"), space1), nom32, terminated(tag(":"), space1))(input)
}

fn numberlist(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list0(space1, nom32)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_numlist() {
        assert_eq!(numberlist("1  2 23    6"),
                   Ok(("", vec![1, 2, 23, 6])))
    }

    #[test]
    fn parse_card() {
        let r = card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert!(r.is_ok());
        assert!(r.unwrap().0.is_empty());
    }

    #[test]
    fn parse_example() {
        let s = fs::read_to_string("example.txt").unwrap();
        let r = parse_input(&s);
        assert!(r.is_ok());
        assert!(r.unwrap().0.is_empty());
    }

    #[test]
    fn example_part_one() {
        let s = fs::read_to_string("example.txt").unwrap();
        let (_, i) = parse_input(&s).unwrap();
        assert_eq!(part_one(&i), 13);
    }

    #[test]
    fn example_part_two() {
        let s = fs::read_to_string("example.txt").unwrap();
        let (_, i) = parse_input(&s).unwrap();
        assert_eq!(part_two(&i), 30);
    }
}
