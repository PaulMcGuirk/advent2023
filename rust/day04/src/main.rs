use std::collections::HashSet;
use std::fs;
use std::time::Instant;

const FILEPATH: &str = "./input/input.txt";

fn count_wins(cards: &Vec<(HashSet<usize>, HashSet<usize>)>) -> Vec<usize> {
    cards
        .iter()
        .map(|(winners, drawn)| winners.intersection(drawn).count())
        .collect::<Vec<_>>()
}

fn score_cards(cards: &Vec<(HashSet<usize>, HashSet<usize>)>) -> usize {
    count_wins(cards)
        .into_iter()
        .map(|count| 1 << count >> 1)
        .sum()
}

fn score_cards_elflike(cards: &Vec<(HashSet<usize>, HashSet<usize>)>) -> usize {
    let wins = count_wins(cards);

    // card_counts[i] = 1 + the number of cards directly one by card i
    let mut card_counts = vec![1; cards.len()];

    for i in (0..cards.len()).rev() {
        let wins = wins[i];
        let max = (i + wins).min(cards.len());
        let new_cards = ((i + 1)..=max).map(|j| card_counts[j]).sum::<usize>();
        card_counts[i] = card_counts[i] + new_cards;
    }

    card_counts.iter().sum()
}

fn parse_input(s: &str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    s.trim()
        .lines()
        .map(|line| {
            let scores = line.split(":").skip(1).next().unwrap();
            let mut nums = scores.split("|").map(|pc| {
                pc.trim()
                    .split_whitespace()
                    .map(|n| n.trim().parse::<usize>().unwrap())
                    .collect::<HashSet<_>>()
            });
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect::<Vec<_>>()
}

fn main() {
    println!("Advent of Code 2023");
    println!("Day 4: Scratchcards");

    let now = Instant::now();

    let input = fs::read_to_string(FILEPATH).expect("Could not read file");

    let cards = parse_input(&input);

    let part_one = score_cards(&cards);
    let part_two = score_cards_elflike(&cards);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);

    println!("Elasped time: {}ms", now.elapsed().as_millis());
}
