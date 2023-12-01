pub mod trie;
use crate::trie::Trie;
use std::io::{BufRead, BufReader};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let i = [
        "one1", "two2", "three3", "four4", "five5", "six6", "seven7", "eight8", "nine9",
    ];
    let trie = Trie::new(&i);
    let sum = buffer.lines().fold(0, |mut acc, line| {
        if let Ok(line) = line {
            let mut first = 0;
            for (i, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    first = c.to_digit(10).unwrap() * 10;
                    break;
                } else if let Some(v) = trie.try_find(&line[i..]) {
                    first = v as u32 * 10;
                    break;
                }
            }
            let mut last = 0;
            for (i, c) in line.chars().rev().enumerate() {
                if c.is_numeric() {
                    last = c.to_digit(10).unwrap();
                    break;
                } else if let Some(v) = trie.try_find(&line[line.len() - i - 1..]) {
                    last = v as u32;
                    break;
                }
            }
            acc += first;
            acc + last
        } else {
            acc
        }
    });
    println!("The calibration falues add up to: {}", sum)
}

#[test]
fn test_trie() {
    let i = vec![
        "one1", "two2", "three3", "four4", "five5", "six6", "seven7", "eight8", "nine9",
    ];
    let trie = Trie::new(&i);
    println!("{trie:#?}");
    let r = trie.try_find("twoefewubno");
    println!("{r:?}");
}
