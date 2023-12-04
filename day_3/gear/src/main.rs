use std::io::{BufRead, BufReader};

/// The SearchResult describes the result given by searching a slice for a number.
/// Success indicates a number was found that is adjacent to a symbol if a gear was found
/// it's position is returned as well.
#[derive(Debug, Clone)]
enum SearchResult {
    Success {
        number: u32,
        next_index: usize,
        gear: Option<(usize, usize)>,
    },
    Failure {
        next_index: usize,
    },
    EOL,
}

/// The GearMap tracks all gears.
struct GearMap {
    pub inner: Vec<Vec<Option<Gear>>>,
}

impl GearMap {
    pub fn new(line_count: usize, line_len: usize) -> Self {
        Self {
            inner: vec![vec![None; line_len]; line_count],
        }
    }
}

/// A Gear can counts the numbers it is adjacent with if there are more than two adjacent
/// numbers the Gear is treated as poisioned.
#[derive(Clone, Copy, Debug)]
struct Gear {
    pub numbers: [Option<u32>; 2],
    pub poisioned: bool,
}

impl Gear {
    pub fn new(number: u32) -> Self {
        Self {
            numbers: [Some(number), None],
            poisioned: false,
        }
    }

    pub fn add_number(&mut self, number: u32) {
        if self.numbers[1].is_none() {
            self.numbers[1] = Some(number)
        } else {
            self.poisioned = true
        }
    }
}

fn main() {
    let input = std::fs::File::open("./input").unwrap();
    let buffer = BufReader::new(input);
    let lines: Vec<Vec<u8>> = buffer
        .lines()
        .filter_map(|l| match l {
            Ok(l) => Some(l.into_bytes()),
            Err(_) => None,
        })
        .collect();
    let mut sum = 0;
    let mut sym_map = GearMap::new(lines.len(), lines[0].len());
    for i in 0..lines.len() {
        let mut j = 0;
        // Look for numbers in line until EOL reached.
        loop {
            match search_number(&lines, i, j) {
                SearchResult::Success {
                    number,
                    next_index,
                    gear: symbol,
                } => {
                    sum += number;
                    j = next_index;
                    if let Some(s) = symbol {
                        if let Some(sym) = &mut sym_map.inner[s.0][s.1] {
                            sym.add_number(number)
                        } else {
                            sym_map.inner[s.0][s.1] = Some(Gear::new(number))
                        }
                    }
                }
                SearchResult::Failure {
                    next_index: last_index,
                } => {
                    j = last_index;
                }
                SearchResult::EOL => break,
            }
        }
    }
    let sym_val = sym_map.inner.iter().flatten().fold(0, |acc, s| {
        if let Some(s) = s {
            if s.poisioned || s.numbers[1].is_none() {
                return acc;
            }
            acc + s.numbers[0].unwrap() * s.numbers[1].unwrap()
        } else {
            acc
        }
    });

    println!("{sum}");
    println!("{sym_val}")
}

enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

/// Finds next number in line and checks if it is adjacent to any symbol
/// and returns the appropriate SearchResult.
fn search_number(data: &[Vec<u8>], i: usize, j: usize) -> SearchResult {
    use Direction::*;
    let Some(start_number) = data[i][j..].iter().position(|e| e.is_ascii_digit()).map(|i| i + j) else {
        return SearchResult::EOL;
    };
    let end_number = data[i][start_number..]
        .iter()
        .position(|e| !e.is_ascii_digit())
        .map(|i| i + start_number)
        .unwrap_or(data.len());
    let mut symbol_found = false;
    let mut screw_idx = None;
    for d in [Top, Bottom, Left, Right] {
        match d {
            Top => {
                if i == 0 {
                    continue;
                }
                match check_top(
                    &data[i - 1],
                    (start_number).checked_sub(1).unwrap_or(0),
                    end_number + 1,
                ) {
                    (true, Some(y)) => {
                        symbol_found = true;
                        screw_idx = Some((i - 1, y));
                    }
                    (true, None) => symbol_found = true,
                    _ => {}
                }
            }
            Bottom => {
                if i == data.len() - 1 {
                    continue;
                }
                match check_bot(
                    &data[i + 1],
                    (start_number).checked_sub(1).unwrap_or(0),
                    end_number + 1,
                ) {
                    (true, Some(y)) => {
                        symbol_found = true;
                        screw_idx = Some((i + 1, y));
                    }
                    (true, None) => symbol_found = true,
                    _ => {}
                }
            }
            Left => {
                if start_number == 0 {
                    continue;
                }
                match check_position(&data[i], start_number - 1) {
                    (true, Some(y)) => {
                        symbol_found = true;
                        screw_idx = Some((i, y));
                    }
                    (true, None) => symbol_found = true,
                    _ => {}
                }
            }
            Right => {
                if end_number == data[i].len() - 1 {
                    continue;
                }
                match check_position(&data[i], end_number) {
                    (true, Some(y)) => {
                        symbol_found = true;
                        screw_idx = Some((i, y));
                    }
                    (true, None) => symbol_found = true,
                    _ => {}
                }
            }
        }
    }
    if !symbol_found {
        return SearchResult::Failure {
            next_index: end_number,
        };
    }

    // Convert askii number to u32
    let number = data[i][start_number..end_number]
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| {
            acc + (*b - 48) as u32 * (10_u32.pow(i as u32))
        });

    SearchResult::Success {
        number,
        next_index: end_number,
        gear: screw_idx,
    }
}

fn check_top(data: &[u8], start: usize, end: usize) -> (bool, Option<usize>) {
    return data[start..end.min(data.len() - 1)]
        .iter()
        .enumerate()
        .fold((false, None), |mut r, (i, b)| {
            if *b != b'.' && !b.is_ascii_digit() {
                if *b == b'*' {
                    return (true, Some(i + start));
                } else {
                    r.0 = true;
                    return r;
                }
            }
            r
        });
}

fn check_bot(data: &[u8], start: usize, end: usize) -> (bool, Option<usize>) {
    return data[start..end.min(data.len() - 1)]
        .iter()
        .enumerate()
        .fold((false, None), |mut r, (i, b)| {
            if *b != b'.' && !b.is_ascii_digit() {
                if *b == b'*' {
                    return (true, Some(i + start));
                } else {
                    r.0 = true;
                    return r;
                }
            }
            r
        });
}

fn check_position(line: &[u8], p: usize) -> (bool, Option<usize>) {
    let p = p.min(line.len() - 1);
    let b = line[p];
    if b != b'.' && !b.is_ascii_digit() {
        if b == b'*' {
            return (true, Some(p));
        } else {
            return (true, None);
        }
    }
    return (false, None);
}
