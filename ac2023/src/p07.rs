use std::collections::HashMap;

use nom::{
    branch::alt,
    character::{
        complete,
        complete::{char, newline, space1},
    },
    combinator::{all_consuming, map, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

fn card(input: &str) -> IResult<&str, Card> {
    alt((
        value(Card::Two, char('2')),
        value(Card::Three, char('3')),
        value(Card::Four, char('4')),
        value(Card::Five, char('5')),
        value(Card::Six, char('6')),
        value(Card::Seven, char('7')),
        value(Card::Eight, char('8')),
        value(Card::Nine, char('9')),
        value(Card::Ten, char('T')),
        value(Card::Jack, char('J')),
        value(Card::Queen, char('Q')),
        value(Card::King, char('K')),
        value(Card::Ace, char('A')),
    ))(input)
}

fn cards(input: &str) -> IResult<&str, [Card; 5]> {
    map(tuple((card, card, card, card, card)), <[Card; 5]>::from)(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s, o)| s.cmp(o))
                .find(|ord| !matches!(ord, std::cmp::Ordering::Equal))
                .unwrap_or(std::cmp::Ordering::Equal),
            x => x,
        }
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counts = HashMap::new();
        for card in self.cards {
            *counts.entry(card).or_insert(0usize) += 1;
        }
        match counts.values().max().expect("no cards") {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if (counts.values().any(|x| x == &2)) => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if (counts.values().filter(|x| x == &&2).count() == 2) => HandType::TwoPair,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

fn hand(input: &str) -> IResult<&str, Hand> {
    map(
        separated_pair(cards, space1, complete::u32),
        |(cards, bid)| Hand { cards, bid },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    all_consuming(terminated(separated_list1(newline, hand), newline))(input)
}

pub fn part1(input: &str) -> String {
    let (_, mut hands) = parse(input).expect("parsing failed");
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card2 {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

fn card2(input: &str) -> IResult<&str, Card2> {
    alt((
        value(Card2::Two, char('2')),
        value(Card2::Three, char('3')),
        value(Card2::Four, char('4')),
        value(Card2::Five, char('5')),
        value(Card2::Six, char('6')),
        value(Card2::Seven, char('7')),
        value(Card2::Eight, char('8')),
        value(Card2::Nine, char('9')),
        value(Card2::Ten, char('T')),
        value(Card2::Joker, char('J')),
        value(Card2::Queen, char('Q')),
        value(Card2::King, char('K')),
        value(Card2::Ace, char('A')),
    ))(input)
}

fn cards2(input: &str) -> IResult<&str, [Card2; 5]> {
    map(
        tuple((card2, card2, card2, card2, card2)),
        <[Card2; 5]>::from,
    )(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand2 {
    cards: [Card2; 5],
    bid: u32,
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            std::cmp::Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .map(|(s, o)| s.cmp(o))
                .find(|ord| !matches!(ord, std::cmp::Ordering::Equal))
                .unwrap_or(std::cmp::Ordering::Equal),
            x => x,
        }
    }
}

impl Hand2 {
    fn get_type(&self) -> HandType {
        let mut counts = HashMap::new();
        for card in self.cards {
            *counts.entry(card).or_insert(0usize) += 1;
        }
        let jokers = counts.remove(&Card2::Joker).unwrap_or(0);
        match counts.values().max().unwrap_or(&0) + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if (counts.values().any(|x| x == &3) && (counts.values().any(|x| x == &2)))
                || (counts.values().filter(|x| x == &&2).count() == 2 && jokers == 1) =>
            {
                HandType::FullHouse
            }
            3 => HandType::ThreeOfAKind,
            2 if (counts.values().filter(|x| x == &&2).count() == 2) => HandType::TwoPair,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn hand2(input: &str) -> IResult<&str, Hand2> {
    map(
        separated_pair(cards2, space1, complete::u32),
        |(cards, bid)| Hand2 { cards, bid },
    )(input)
}

fn parse2(input: &str) -> IResult<&str, Vec<Hand2>> {
    all_consuming(terminated(separated_list1(newline, hand2), newline))(input)
}

pub fn part2(input: &str) -> String {
    let (_, mut hands) = parse2(input).expect("parsing failed");
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 6440.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 5905.to_string());
    }
}
