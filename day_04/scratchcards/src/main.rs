mod card;
use crate::card::Card;
use std::io::{BufRead, BufReader};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let mut cards: Vec<Card> = Vec::new();
    for line in buffer.lines() {
        let Ok(line) = line else {
            continue;
        };
        cards.push(line.into())
    }
    let res: u32 = cards.iter().map(|c| c.calc_winnings()).sum();
    println!("{res}");
    for i in 0..cards.len() {
        let (cur, fut) = cards.split_at_mut(i + 1);
        let current = cur.last().unwrap();
        let amount = current.amount;
        let winnings = current.calc_won_scratch();
        let rest_len = fut.len();
        fut[..(winnings as usize).min(rest_len)]
            .iter_mut()
            .for_each(|c| c.amount += amount);
    }
    println!("{}", cards.iter().fold(0, |acc, c| acc + c.amount))
}
