use std::str::FromStr;
use std::{cmp::Ordering, collections::HashMap};

fn card_weight(card: char) -> u8 {
    match card {
        'T' => 10,
        'J' => 1, // in part 2, J is the weakest card
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

struct Hand {
    pub hand_type: HandType,
    pub cards: [char; 5],
    pub bid: u32,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').expect("expected `<cards> <bid>`");
        let cards = cards
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .expect("expected 5 cards");
        let bid = bid.parse().expect("expected bid to be a positive integer");
        Ok(Hand::new(cards, bid))
    }
}

impl Hand {
    pub fn new(cards: [char; 5], bid: u32) -> Hand {
        Hand {
            cards,
            bid,
            hand_type: Hand::hand_type(cards),
        }
    }

    fn hand_type(cards: [char; 5]) -> HandType {
        let mut card_counts = HashMap::new();
        for card in cards.iter() {
            if card != &'J' {
                let count = card_counts.entry(card).or_insert(0);
                *count += 1;
            }
        }

        // these counts don't include jokers
        let mut same_counts = card_counts.values().collect::<Vec<_>>();
        same_counts.sort_unstable();
        match same_counts.as_slice() {
            [5] | [4] | [3] | [2] | [1] | [] => HandType::FiveOfAKind,
            [1, 4] | [1, 3] | [1, 2] | [1, 1] => HandType::FourOfAKind,
            [2, 3] | [2, 2] => HandType::FullHouse,
            [1, 1, 3] | [1, 1, 2] | [1, 1, 1] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 1, 2] | [1, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Invalid hand: {:?} ({:?})", cards, same_counts),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            let self_cards = self.cards.iter().map(|c| card_weight(*c));
            let other_cards = other.cards.iter().map(|c| card_weight(*c));
            self_cards.cmp(other_cards)
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

/// Ordered weakest to strongest
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn part2() {
    // let (input, expected_winnings) = (include_str!("sample.txt"), Some(5905));
    let (input, expected_winnings) = (include_str!("my_input.txt"), Some(251003917));
    let mut hands = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.parse::<Hand>()
                .unwrap_or_else(|_| panic!("line {}: invalid hand/bid", i))
        })
        .collect::<Vec<_>>();
    hands.sort_unstable();

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| (i as u32 + 1) * hand.bid + acc);

    println!("Part 2: {}", winnings);
    if let Some(expected_winnings) = expected_winnings {
        assert_eq!(winnings, expected_winnings);
    }
}
