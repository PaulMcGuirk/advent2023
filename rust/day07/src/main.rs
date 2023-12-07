use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Eq, PartialEq)]
struct Hand {
    type_: u32,
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn from_str(s: &str, jokers: bool) -> Self {
        let mut pcs = s.trim().split_whitespace();

        let mut cards = vec![];
        let mut counts = vec![0u32; 15];

        for card in pcs.next().unwrap().chars() {
            let r = Self::parse_card(card, jokers);
            cards.push(r);
            counts[r as usize] += 1;
        }

        // println!("{}", s);
        // println!("{:?}", counts);

        let type_ = if jokers {
            Self::get_hand_type_with_jokers(counts)
        } else {
            Self::get_hand_type(counts)
        };

        // if jokers {
        //     println!("{}", s);
        //     println!("{}", type_);
        // }

        let bid = pcs.next().unwrap().parse::<u32>().unwrap();

        Self { type_, cards, bid }
    }

    fn parse_card(card: char, jokers: bool) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {
                if jokers {
                    0
                } else {
                    11
                }
            }
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }

    fn get_hand_type(counts: Vec<u32>) -> u32 {
        let count_counts = (0..=5)
            .map(|c| counts.iter().filter(|&&c2| c2 == c).count())
            .collect::<Vec<_>>();

        // println!("{:?}", count_counts);

        let rank = if count_counts[5] == 1 {
            6
        } else if count_counts[4] == 1 {
            5
        } else if count_counts[3] == 1 {
            3 + count_counts[2]
        } else {
            count_counts[2]
        };

        // println!("{}", rank);

        rank as u32
    }

    fn get_hand_type_with_jokers(counts: Vec<u32>) -> u32 {
        if counts[0] == 0 {
            Self::get_hand_type(counts)
        } else {
            // println!("jokers!");
            (2..=14)
                .filter_map(|repl| {
                    if repl == 11 {
                        None
                    } else {
                        let mut new_counts = counts.clone();
                        new_counts[0] = new_counts[0] - 1;
                        new_counts[repl] = new_counts[repl] + 1;
                        Some(Self::get_hand_type_with_jokers(new_counts))
                    }
                })
                .max()
                .unwrap()
        }
    }
    // let count_counts = (0..=5)
    //     .map(|c| counts.iter().filter(|&&c2| c2 == c).count())
    //     .collect::<Vec<_>>();

    // println!("{:?}", count_counts);

    // let rank = if count_counts[5] == 1 {
    //     6
    // } else if count_counts[4] == 1 {
    //     5
    // } else if count_counts[3] == 1 {
    //     3 + count_counts[2]
    // } else {
    //     count_counts[2]
    // };

    // println!("{}", rank);

    // rank as u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.type_.cmp(&other.type_) {
            std::cmp::Ordering::Equal => (0..5)
                .find_map(|i| match self.cards[i].cmp(&other.cards[i]) {
                    Ordering::Equal => None,
                    ord => Some(ord),
                })
                .unwrap_or(Ordering::Equal),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_winnings(hands: &Vec<Hand>) -> u32 {
    let mut hands = hands.iter().map(|c| c).collect::<Vec<_>>();
    hands.sort();

    // for hand in hands.iter() {
    //     println!("{} {}", hand.bid, hand.type_);
    // }

    hands
        .iter()
        .enumerate()
        .map(|(i, &hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn parse_input(raw_input: &str, jokers: bool) -> Vec<Hand> {
    raw_input
        .trim()
        .lines()
        .map(|ln| Hand::from_str(ln, jokers))
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 7: Camel Cards");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    // let raw_input = "32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483";
    let hands = parse_input(&raw_input, false);

    let part_one = get_winnings(&hands);

    let hands = parse_input(&raw_input, true);

    let part_two = get_winnings(&hands);

    // let part_one = races
    //     .iter()
    //     .map(|&(t, d)| count_wins(t, d))
    //     .product::<u64>();
    // let part_two = count_wins(concat_time, concat_dist);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
