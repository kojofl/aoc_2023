use cube::Game;
use std::io::{BufRead, BufReader};

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let g = Game {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum: u32 = buffer
        .lines()
        .filter_map(|l| {
            let l = l.expect("Valid UTF-8 String");
            let (id, l) = l
                .split_once(':')
                .expect("Game id to be seperated from draws by :");
            if line_possible(l, &g) {
                Some(extract_id(id))
            } else {
                None
            }
        })
        .sum();
    println!("{sum}");
}

fn line_possible(line: &str, g: &Game) -> bool {
    for pulls in line.split(';') {
        if !pulls.split(',').all(|p| g.is_plausible(p)) {
            return false;
        }
    }
    true
}

fn extract_id(game: &str) -> u32 {
    let (_, id) = game.split_once(' ').expect("Game {id}");
    id.parse().unwrap()
}
