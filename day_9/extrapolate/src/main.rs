use std::io::{BufRead, BufReader};

fn main() {
    let file = std::fs::File::open("input");
    let reader = BufReader::new(file.unwrap());
    let number_rows = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let sum = number_rows.iter().fold(0, |acc, r| acc + extrapolate(r));
    println!("{sum}")
}

pub fn extrapolate(row: &[i32]) -> i32 {
    if row.iter().all(|e| *e == 0) {
        return 0;
    }
    let mut dif_row = Vec::with_capacity(row.len());
    for win in row.windows(2) {
        dif_row.push(win[1] - win[0]);
    }
    return row.last().unwrap() + extrapolate(&dif_row);
}
