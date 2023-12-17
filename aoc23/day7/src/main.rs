use anyhow::{anyhow, Result};
use counter::Counter;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

fn card_value(value: char) -> Result<u8> {
    Ok(match value {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        _ => value
            .to_digit(10)
            .ok_or_else(|| anyhow!("Invalid card identifier: {}", value))? as u8,
    })
}

#[derive(Debug)]
struct Hand {
    hand: [u8; 5],
    bid: i64,
}

impl Hand {
    fn parse(hand: &str, bid: &str) -> Result<Hand> {
        let bid = bid.parse::<i64>()?;
        let hand: Vec<u8> = hand
            .chars()
            .map(|c| card_value(c))
            .collect::<Result<Vec<u8>>>()?;
        let hand: &[u8] = hand
            .chunks_exact(5)
            .next()
            .ok_or_else(|| anyhow!("Short hand: {}", hand.len()))?;
        Ok(Hand {
            hand: hand.try_into()?,
            bid,
        })
    }
}

#[derive(Debug)]
struct EvaluatedHand<'a> {
    hand: &'a Hand,
    counts: [u8; 5],
}

impl EvaluatedHand<'_> {
    fn evaluate(hand: &Hand) -> EvaluatedHand {
        // map of card value to number of that card
        let card_counts: Counter<u8> = hand.hand.iter().cloned().collect();

        // counts[i] is the number of different card values that we have i of
        // for example, two pair has counts[2] == 2 and counts[1] == 1
        let mut counts = [0; 5];
        for (_, &count) in card_counts.iter().filter(|(v, _)| **v != 0u8) {
            counts[5 - count] += 1;
        }

        // Make all jokers contribute to whatever card we have the most of.
        if let Some(&jokers) = card_counts.get(&0u8) {
            if let Some(highest_ct) = card_counts
                .iter()
                .filter(|(v, _)| **v != 0u8)
                .map(|(_, c)| c)
                .max()
            {
                counts[5 - *highest_ct] -= 1;
                counts[5 - (*highest_ct + jokers)] += 1;
            } else {
                counts[0] = 1;
            }
        }

        EvaluatedHand { hand, counts }
    }
}

impl PartialEq<Self> for EvaluatedHand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.hand.hand.eq(&other.hand.hand)
    }
}

impl PartialOrd<Self> for EvaluatedHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for EvaluatedHand<'_> {}

impl Ord for EvaluatedHand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.counts
            .cmp(&other.counts)
            .then(self.hand.hand.cmp(&other.hand.hand))
    }
}

fn main() {
    let input_s = fs::read_to_string("inputs/day7.txt").unwrap();
    let input = parse_input(&input_s).unwrap();
    println!("Part two: {}", part_two(input));
}

fn part_two(input: Vec<Hand>) -> i64 {
    let mut hands: Vec<EvaluatedHand> = input.iter().map(|h| EvaluatedHand::evaluate(h)).collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as i64 * hand.hand.bid)
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Hand>> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (hand, bid) = line
                .split_whitespace()
                .next_tuple()
                .ok_or_else(|| anyhow!("Short input line: {}", line))?;
            Ok(Hand::parse(hand, bid)?)
        })
        .collect()
}
