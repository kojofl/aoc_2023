use std::io::{BufRead, BufReader};

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut cols: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for (i, b) in line.bytes().enumerate() {
            if i >= cols.len() {
                cols.push(vec![b]);
            } else {
                cols[i].push(b);
            }
        }
    }
    let mut sum = 0;
    for col in cols {
        let mut current_points = col.len();
        sum +=
            col.iter()
                .enumerate()
                .filter(|(_, b)| **b != b'.')
                .fold(0, |mut acc, (i, b)| match b {
                    b'O' => {
                        acc += current_points;
                        current_points = current_points.saturating_sub(1);
                        acc
                    }
                    b'#' => {
                        current_points = col.len() - i - 1;
                        acc
                    }
                    _ => panic!("unknown rock {}", b),
                })
    }
    println!("{sum}")
}
