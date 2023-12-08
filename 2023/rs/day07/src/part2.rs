use std::{cmp::Ordering, collections::HashMap};

use phf::phf_map;

use crate::custom_error::AocError;

static CARD_PRIORITIES: phf::Map<char, u8> = phf_map! {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'T' => 10,
    '9' => 9,
    '8' => 8,
    '7' => 7,
    '6' => 6,
    '5' => 5,
    '4' => 4,
    '3' => 3,
    '2' => 2,
    'J' => 1,
};

#[derive(Eq, PartialEq, Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn priority(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: [char; 5],
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_frequencies = HashMap::new();
        for card in self.cards.iter() {
            if !card_frequencies.contains_key(card) {
                card_frequencies.insert(card, 0);
            }
            card_frequencies.get_mut(card).map(|val| *val = &*val + 1);
        }

        let jokers = *card_frequencies.get(&'J').unwrap_or(&0);
        if jokers > 0 && jokers < 5 {
            let highest_count_card = card_frequencies
                .iter()
                .filter(|(k, _)| ***k != 'J')
                .max_by(|a, b| a.1.cmp(&b.1))
                .map(|(k, _)| k)
                .unwrap()
                .to_owned();
            card_frequencies
                .get_mut(highest_count_card)
                .map(|val| *val = &*val + jokers);
            card_frequencies.remove(&'J');
        }

        if card_frequencies.len() == 1 {
            HandType::FiveOfAKind
        } else if *card_frequencies.values().max().unwrap() == 4 {
            HandType::FourOfAKind
        } else if *card_frequencies.values().max().unwrap() == 3 {
            if *card_frequencies.values().min().unwrap() == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if card_frequencies.values().filter(|val| **val == 2).count() == 2 {
            HandType::TwoPair
        } else if *card_frequencies.values().max().unwrap() == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(CARD_PRIORITIES[&self.cards[0]].cmp(&CARD_PRIORITIES[&other.cards[0]]))
            .then(CARD_PRIORITIES[&self.cards[1]].cmp(&CARD_PRIORITIES[&other.cards[1]]))
            .then(CARD_PRIORITIES[&self.cards[2]].cmp(&CARD_PRIORITIES[&other.cards[2]]))
            .then(CARD_PRIORITIES[&self.cards[3]].cmp(&CARD_PRIORITIES[&other.cards[3]]))
            .then(CARD_PRIORITIES[&self.cards[4]].cmp(&CARD_PRIORITIES[&other.cards[4]]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn process(input: &str) -> std::result::Result<String, AocError> {
    let mut hands: Vec<Hand> = input.lines().map(|line| parse_line(line)).collect();
    hands.sort();
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, hand)| -> u32 { hand.bid * (1 + u32::try_from(i).unwrap()) })
        .sum::<u32>()
        .to_string())
}

fn parse_line(line: &str) -> Hand {
    let mut parts = line.split(" ");
    let mut cards = [char::default(); 5];
    for (i, ch) in parts.next().unwrap().chars().enumerate() {
        cards[i] = ch;
    }
    let bid = parts.next().unwrap().trim().parse().unwrap();
    Hand { cards, bid }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let want = "5905";
        match process(input) {
            Ok(r) => assert_eq!(want, r),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
