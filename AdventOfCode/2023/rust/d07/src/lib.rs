use core::fmt;
use std::{cmp, fmt::{Display, Formatter}};

use nom::{IResult, character::complete::{line_ending, space1, self}, bytes::complete::take_until};

mod part2;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Display for HandType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HandType::HighCard => write!(f, "High Card"),
            HandType::OnePair => write!(f, "One Pair"),
            HandType::TwoPair => write!(f, "Two Pair"),
            HandType::ThreeOfAKind => write!(f, "Three of a Kind"),
            HandType::FullHouse => write!(f, "Full House"),
            HandType::FourOfAKind => write!(f, "Four of a Kind"),
            HandType::FiveOfAKind => write!(f, "Five of a Kind"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    wager: u32,
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (HandType::HighCard, HandType::HighCard) => cmp::Ordering::Equal,
            (HandType::HighCard, _) => cmp::Ordering::Less,
            (_, HandType::HighCard) => cmp::Ordering::Greater,
            (HandType::OnePair, HandType::OnePair) => cmp::Ordering::Equal,
            (HandType::OnePair, _) => cmp::Ordering::Less,
            (_, HandType::OnePair) => cmp::Ordering::Greater,
            (HandType::TwoPair, HandType::TwoPair) => cmp::Ordering::Equal,
            (HandType::TwoPair, _) => cmp::Ordering::Less,
            (_, HandType::TwoPair) => cmp::Ordering::Greater,
            (HandType::ThreeOfAKind, HandType::ThreeOfAKind) => cmp::Ordering::Equal,
            (HandType::ThreeOfAKind, _) => cmp::Ordering::Less,
            (_, HandType::ThreeOfAKind) => cmp::Ordering::Greater,
            (HandType::FullHouse, HandType::FullHouse) => cmp::Ordering::Equal,
            (HandType::FullHouse, _) => cmp::Ordering::Less,
            (_, HandType::FullHouse) => cmp::Ordering::Greater,
            (HandType::FourOfAKind, HandType::FourOfAKind) => cmp::Ordering::Equal,
            (HandType::FourOfAKind, _) => cmp::Ordering::Less,
            (_, HandType::FourOfAKind) => cmp::Ordering::Greater,
            (HandType::FiveOfAKind, HandType::FiveOfAKind) => cmp::Ordering::Equal,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let hand_compare = self.hand_type.cmp(&other.hand_type);
        if hand_compare != cmp::Ordering::Equal {
            return hand_compare;
        }
        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            let card_compare = card1.cmp(card2);
            if card_compare != cmp::Ordering::Equal {
                return card_compare;
            }
        }
        return cmp::Ordering::Equal;
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: ", self.hand_type)?;
        for card in self.cards.iter() {
            write!(f, "{}", CARDS[*card as usize])?;
        }
        write!(f, " ({})", self.wager)
    }
}

const CARDS: [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];

fn hand_type_from_cards(cards: &Vec<u8>) -> HandType {
    let mut counts = [0; 13];
    for card in cards {
        counts[*card as usize] += 1;
    }
    let mut counts = counts.to_vec();
    counts.sort();
    counts.reverse();
    match counts.as_slice() {
        [1,1,1,1,1, ..] => HandType::HighCard,
        [2,1,1,1, ..] => HandType::OnePair,
        [2, 2, 1, ..] => HandType::TwoPair,
        [3, 1, 1, ..] => HandType::ThreeOfAKind,
        [4,1, ..] => HandType::FourOfAKind,
        [5, ..] => HandType::FiveOfAKind,
        [3,2, ..] => HandType::FullHouse,
        _ => panic!("Invalid hand"),
    }
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (input, wager) = complete::u32(input)?;
    let cards = cards.chars().map(|c| CARDS.iter().position(|&x| x == c).unwrap() as u8).collect();
    let hand_type = hand_type_from_cards(&cards);
    Ok((input, Hand {
        cards,
        hand_type,
        wager,
    }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = nom::multi::separated_list1(line_ending ,parse_hand)(input)?;
    Ok((input, hands))
}

pub fn process_part_1(input: &str) -> u32 {
    let (_, mut hands) = parse_input(input).unwrap();
    hands.sort_by(|a, b| a.cmp(b));
    hands.iter().enumerate().map(|(i, hand)| hand.wager * (i as u32 + 1)).sum::<u32>()
}


pub fn process_part_2(input: &str) -> u32{
    part2::process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PUZZLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_1() {
        assert_eq!(process_part_1(EXAMPLE_PUZZLE_INPUT), 6440, "Failed example 1");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(process_part_2(EXAMPLE_PUZZLE_INPUT), 5905, "Failed example 2");
    }
}

