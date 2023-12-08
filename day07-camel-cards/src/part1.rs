use std::str::FromStr;
use std::{cmp::Ordering, collections::HashMap};

fn card_weight(card: char) -> u8 {
    match card {
        'T' => 10,
        'J' => 11,
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
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        let mut same_counts = card_counts.values().collect::<Vec<_>>();
        same_counts.sort_unstable();
        match same_counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPairs,
            [1, 1, 1, 2] => HandType::OnePair,
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

#[test]
fn test_hand_type() {
    let five_of_a_kind = Hand::new(['A', 'A', 'A', 'A', 'A'], 0);
    let four_of_a_kind = Hand::new(['A', 'A', 'K', 'A', 'A'], 0);
    let full_house = Hand::new(['A', 'A', 'K', 'K', 'K'], 0);
    let three_of_a_kind = Hand::new(['A', 'A', 'K', 'A', 'Q'], 0);
    let two_pairs = Hand::new(['A', 'K', 'K', 'Q', 'A'], 0);
    let one_pair = Hand::new(['J', 'A', 'K', 'Q', 'J'], 0);
    let high_card = Hand::new(['2', '3', '6', 'J', 'T'], 0);
    assert_eq!(HandType::FiveOfAKind, five_of_a_kind.hand_type);
    assert_eq!(HandType::FourOfAKind, four_of_a_kind.hand_type);
    assert_eq!(HandType::FullHouse, full_house.hand_type);
    assert_eq!(HandType::ThreeOfAKind, three_of_a_kind.hand_type);
    assert_eq!(HandType::TwoPairs, two_pairs.hand_type);
    assert_eq!(HandType::OnePair, one_pair.hand_type);
    assert_eq!(HandType::HighCard, high_card.hand_type);

    assert!(HandType::FiveOfAKind > HandType::FourOfAKind);

    assert!(five_of_a_kind > four_of_a_kind);
    assert!(four_of_a_kind > full_house);
    assert!(full_house > three_of_a_kind);
    assert!(three_of_a_kind > two_pairs);
    assert!(two_pairs > one_pair);
    assert!(one_pair > high_card);
}

#[test]
fn test_same_type_ranking() {
    let five_of_a_kind1 = Hand::new(['2', '2', '2', '2', '2'], 0);
    let five_of_a_kind2 = Hand::new(['A', 'A', 'A', 'A', 'A'], 0);

    assert_eq!(five_of_a_kind1.hand_type, HandType::FiveOfAKind);
    assert_eq!(five_of_a_kind2.hand_type, HandType::FiveOfAKind);
    assert!(five_of_a_kind1 < five_of_a_kind2);

    let high_card1 = Hand::new(['7', '2', '3', 'Q', 'T'], 0);
    let high_card2 = Hand::new(['7', '2', '7', 'Q', 'T'], 0);
    assert_eq!(high_card1.hand_type, HandType::HighCard);
    assert!(high_card2 > high_card1); // 7 beats 3 in position 3
}

pub fn part1() {
    // let (input, expected_winnings) = (include_str!("sample.txt"), Some(6440));
    let (input, expected_winnings) = (include_str!("my_input.txt"), None);
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

    println!("Part 1: {}", winnings);
    if let Some(expected_winnings) = expected_winnings {
        assert_eq!(winnings, expected_winnings);
    }
}
