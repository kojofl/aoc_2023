use std::io::BufRead;

fn main() {
    let f = std::fs::File::open("./input").unwrap();
    let reader = std::io::BufReader::new(f);
    println!("{}", part_2(reader));
}

fn part_1(reader: impl BufRead) -> u32 {
    let mut lines = reader.lines();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .zip(lines.next().unwrap().unwrap().split_whitespace())
        .skip(1)
        .map(|(t, d)| (t.parse::<u32>().unwrap(), d.parse::<u32>().unwrap()))
        .map(|(time, dist)| {
            let mut wins = 0;
            for hold in 1..time - 1 {
                let fin = hold * (time - hold);
                if fin > dist {
                    wins += 1;
                } else {
                    if wins > 0 {
                        break;
                    }
                }
            }
            wins
        })
        .product()
}

fn part_2(reader: impl BufRead) -> u64 {
    let mut lines = reader.lines();
    lines
        .next()
        .unwrap()
        .unwrap()
        .split(':')
        .zip(lines.next().unwrap().unwrap().split(':'))
        .skip(1)
        .map(|(t, d)| {
            (
                t.split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
                d.split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .map(|(time, dist)| {
            let mut wins = 0;
            for hold in 1..time - 1 {
                let fin = hold * (time - hold);
                if fin > dist {
                    wins += 1;
                } else {
                    if wins > 0 {
                        break;
                    }
                }
            }
            wins
        })
        .product()
}
