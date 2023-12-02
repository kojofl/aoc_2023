pub mod trie;
use crate::trie::Trie;
use std::io::{BufRead, BufReader};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let trie =
        trie!("one1", "two2", "three3", "four4", "five5", "six6", "seven7", "eight8", "nine9");
    let sum = buffer
        .lines()
        .filter(|l| l.is_ok())
        .fold(0, |mut acc, line| {
            let Ok(line) = line else {
                unreachable!();
            };
            let mut first = 0;
            for i in 0..line.len() {
                if let Some(v) = trie.try_find(&line[i..]) {
                    first = v * 10;
                    break;
                }
            }
            let mut last = 0;
            for i in 0..line.len() {
                if let Some(v) = trie.try_find(&line[line.len() - i - 1..]) {
                    last = v;
                    break;
                }
            }
            acc += first;
            acc + last
        });
    println!("The calibration values add up to: {}", sum)
}

#[test]
fn test_trie() {
    use crate::trie::Trie;
    let i = vec![
        "one1", "two2", "three3", "four4", "five5", "six6", "seven7", "eight8", "nine9",
    ];
    let trie = Trie::new(&i);
    println!("{trie:#?}");
    let r = trie.try_find("twoefewubno");
    println!("{r:?}");
}
