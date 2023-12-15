use cube::Game;
use std::io::{BufRead, BufReader};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let sum = buffer
        .lines()
        .map(|l| {
            let l = l.expect("Valid UTF-8 String");
            let (_, l) = l
                .split_once(':')
                .expect("Game id to be seperated from draws by :");
            let mut game = Game::default();
            for pulls in l.split(';') {
                for pull in pulls.split(',') {
                    game.exchange_if_higher(pull)
                }
            }
            game.power()
        })
        .sum::<u32>();
    println!("{sum}")
}
