use std::io::BufRead;

fn main() {
    let f = std::fs::File::open("./input").unwrap();
    let reader = std::io::BufReader::new(f);
    println!("{}", part_2(reader));
}

fn part_1(reader: impl BufRead) -> u64 {
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
            let (x_1, x_2) = pq(-(time as f64), dist as f64);
            x_2 - x_1
        })
        .product()
}

fn pq(p: f64, q: f64) -> (u64, u64) {
    let x = -(p / 2.0);
    let x_2 = (x + ((p / 2.0).powf(2.0) - q).sqrt()) as u64;
    let x_1 = (x - ((p / 2.0).powf(2.0) - q).sqrt()) as u64;
    (x_1, x_2)
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
        .fold(0, |_, (time, dist)| {
            let (x_1, x_2) = pq(-(time as f64), dist as f64);
            x_2 - x_1
        })
}
