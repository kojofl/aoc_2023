use std::io::{BufRead, BufReader};

use c_part2::{Card, Hand};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let mut hands: Vec<Hand> = Vec::new();
    for line in buffer.lines().map(Result::unwrap) {
        let Some((hand, bid)) = line.split_once(' ') else {
            continue;
        };
        let hand: Vec<Card> = hand.chars().take(5).map(Card::from).collect();
        let bid = bid.parse::<u64>().unwrap();
        hands.push(Hand::new(&hand[..5].try_into().unwrap(), bid));
    }
    hands.sort();
    let s: u64 = hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum();

    println!("{s}");
}
