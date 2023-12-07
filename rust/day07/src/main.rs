use std::cmp::Ordering;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

#[derive(Eq, PartialEq)]
struct Hand {
    score: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn from_str(s: &str, joker: bool) -> Self {
        let pcs = s.trim().split_whitespace().collect::<Vec<_>>();
        let score = Self::score(&pcs[0], joker);
        let bid = pcs[1].parse::<u32>().unwrap();

        Self { score, bid }
    }

    fn score(s: &str, joker: bool) -> Vec<u32> {
        let cards = s
            .chars()
            .map(|ch| Self::parse_card(ch, joker))
            .collect::<Vec<_>>();

        let counts = (0..15)
            .map(|val| cards.iter().filter(|&&c| c == val).count() as u32)
            .collect::<Vec<_>>();

        let type_ = Self::get_hand_type(counts.clone());

        let mut score = vec![type_];
        score.extend(cards.clone());

        score
    }

    fn parse_card(card: char, joker: bool) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if joker => 0,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }

    fn get_hand_type(counts: Vec<u32>) -> u32 {
        if counts[0] == 0 {
            // no jokers
            let count_counts = (0..=5)
                .map(|c| counts.iter().filter(|&&c2| c2 == c).count())
                .collect::<Vec<_>>();

            let type_ = if count_counts[5] == 1 {
                6
            } else if count_counts[4] == 1 {
                5
            } else if count_counts[3] == 1 {
                3 + count_counts[2]
            } else {
                count_counts[2]
            };

            type_ as u32
        } else {
            // handle jokers
            (2..=14)
                .filter_map(|repl| {
                    if repl == 11 {
                        None
                    } else {
                        let new_counts = counts
                            .iter()
                            .enumerate()
                            .map(|(i, &ct)| match i {
                                0 => ct - 1,
                                i if i == repl => ct + 1,
                                _ => ct,
                            })
                            .collect::<Vec<_>>();
                        Some(Self::get_hand_type(new_counts))
                    }
                })
                .max()
                .unwrap()
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        (0..6)
            .find_map(|i| match self.score[i].cmp(&other.score[i]) {
                Ordering::Equal => None,
                ord => Some(ord),
            })
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_winnings(hands: &Vec<Hand>) -> u32 {
    let mut hands = hands.iter().map(|h| h).collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, &hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn parse_input(raw_input: &str) -> (Vec<Hand>, Vec<Hand>) {
    let mut standard = vec![];
    let mut with_joker = vec![];

    for ln in raw_input.trim().lines() {
        standard.push(Hand::from_str(ln, false));
        with_joker.push(Hand::from_str(ln, true));
    }

    (standard, with_joker)
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 7: Camel Cards");

    let now = Instant::now();

    let raw_input = fs::read_to_string(FILEPATH).expect("Could not read file");
    let (standard_hands, joker_hands) = parse_input(&raw_input);

    let part_one = get_winnings(&standard_hands);
    let part_two = get_winnings(&joker_hands);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
