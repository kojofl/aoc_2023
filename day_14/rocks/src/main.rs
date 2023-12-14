use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut rocks: Vec<(usize, usize)> = Vec::new();
    let mut r_block: Vec<Vec<usize>> = Vec::new();
    let mut c_block: Vec<Vec<usize>> = Vec::new();
    let mut colls = 0;
    let mut rows = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        r_block.push(Vec::new());
        colls = line.len();
        for (j, b) in line.bytes().enumerate() {
            if j >= c_block.len() {
                c_block.push(Vec::new());
            }
            if b == b'O' {
                rocks.push((i, j))
            } else if b == b'#' {
                r_block[i].push(j);
                c_block[j].push(i)
            }
        }
        rows += 1;
    }
    let mut mem = HashMap::new();
    let mut cache_cycle: Vec<State> = Vec::new();
    let mut left_to_run = 0;
    let mut cycle_found = false;
    for i in 0..1000000000 {
        let (c, c_k) = rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Top,
            &mut mem,
        );
        if c {
            let cache = c_k.unwrap();
            if let Some(first) = cache_cycle.first() {
                if *first == cache {
                    cycle_found = true;
                } else if !cycle_found {
                    cache_cycle.push(cache);
                }
            } else {
                cache_cycle.push(cache);
            }
        }
        let (c, c_k) = rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Left,
            &mut mem,
        );
        if c {
            let cache = c_k.unwrap();
            if let Some(first) = cache_cycle.first() {
                if *first == cache {
                    cycle_found = true;
                } else if !cycle_found {
                    cache_cycle.push(cache);
                }
            } else {
                cache_cycle.push(cache);
            }
        }
        let (c, c_k) = rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Down,
            &mut mem,
        );
        if c {
            let cache = c_k.unwrap();
            if let Some(first) = cache_cycle.first() {
                if *first == cache {
                    cycle_found = true;
                } else if !cycle_found {
                    cache_cycle.push(cache);
                }
            } else {
                cache_cycle.push(cache);
            }
        }
        let (c, c_k) = rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Right,
            &mut mem,
        );
        if c {
            let cache = c_k.unwrap();
            if let Some(first) = cache_cycle.first() {
                if *first == cache {
                    cycle_found = true;
                } else if !cycle_found {
                    cache_cycle.push(cache);
                }
            } else {
                cache_cycle.push(cache);
            }
        }
        if cycle_found {
            left_to_run = (1000000000 - i) % cache_cycle.len() - 1;
            break;
        }
    }
    for _ in 0..left_to_run {
        rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Top,
            &mut mem,
        );
        rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Left,
            &mut mem,
        );
        rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Down,
            &mut mem,
        );
        rotatierer(
            &mut rocks,
            &r_block,
            &c_block,
            colls,
            rows,
            Direction::Right,
            &mut mem,
        );
    }

    let w = calc_weight(&rocks, colls);
    println!("{w}")
}
fn calc_weight(rocks: &[(usize, usize)], len: usize) -> usize {
    rocks.iter().fold(0, |acc, (r, _)| acc + (len - r))
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Top,
    Down,
    Left,
    Right,
}

type State = (Vec<(usize, usize)>, Direction);

fn rotatierer(
    rocks: &mut [(usize, usize)],
    bound_r: &[Vec<usize>],
    bound_c: &[Vec<usize>],
    cols: usize,
    rows: usize,
    rotation: Direction,
    mem: &mut HashMap<State, Vec<(usize, usize)>>,
) -> (bool, Option<State>) {
    match rotation {
        Direction::Top | Direction::Down => rocks.sort_by(|a, b| match a.1.cmp(&b.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            o => o,
        }),
        Direction::Left | Direction::Right => rocks.sort_by(|a, b| match a.0.cmp(&b.0) {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            o => o,
        }),
    }
    let state = (rocks.to_vec(), rotation);
    if let Some(seen) = mem.get(&state) {
        rocks.copy_from_slice(&seen[..]);
        return (true, Some(state.clone()));
    }

    match rotation {
        Direction::Top => {
            let mut free = 0;
            let mut current_col_index = 0;
            'rock: for rock in rocks.iter_mut() {
                if current_col_index != rock.1 {
                    free = 0;
                    current_col_index = rock.1;
                }
                let blocking = &bound_c[current_col_index];
                for block in blocking {
                    if (free..*block).contains(&rock.0) {
                        rock.0 = free;
                        free += 1;
                        continue 'rock;
                    }
                    free = free.max(block + 1);
                }
                if (free..).contains(&rock.0) {
                    rock.0 = free;
                    free += 1;
                }
            }
        }
        Direction::Down => {
            let mut free = rows - 1;
            let mut current_col_index = 0;
            'rock: for rock in rocks.iter_mut().rev() {
                if current_col_index != rock.1 {
                    free = rows - 1;
                    current_col_index = rock.1;
                }
                let blocking = &bound_c[current_col_index];
                for block in blocking.iter().rev() {
                    if (*block..=free).contains(&rock.0) {
                        rock.0 = free;
                        free -= 1;
                        continue 'rock;
                    }
                    free = free.min(block.saturating_sub(1));
                }
                if (..=free).contains(&rock.0) {
                    rock.0 = free;
                    free = free.saturating_sub(1);
                }
            }
        }
        Direction::Left => {
            let mut free = 0;
            let mut current_row_index = 0;
            'rock: for rock in rocks.iter_mut() {
                if current_row_index != rock.0 {
                    free = 0;
                    current_row_index = rock.0;
                }
                let blocking = &bound_r[current_row_index];
                for block in blocking {
                    if (free..*block).contains(&rock.1) {
                        rock.1 = free;
                        free += 1;
                        continue 'rock;
                    }
                    free = free.max(block + 1);
                }
                if (free..).contains(&rock.1) {
                    rock.1 = free;
                    free += 1;
                }
            }
        }
        Direction::Right => {
            let mut free = cols - 1;
            let mut current_row_index = 0;
            'rock: for rock in rocks.iter_mut().rev() {
                if current_row_index != rock.0 {
                    free = cols - 1;
                    current_row_index = rock.0;
                }
                let blocking = &bound_r[current_row_index];
                for block in blocking.iter().rev() {
                    if (*block..=free).contains(&rock.1) {
                        rock.1 = free;
                        free -= 1;
                        continue 'rock;
                    }
                    free = free.min(block.saturating_sub(1));
                }
                if (..=free).contains(&rock.1) {
                    rock.1 = free;
                    free = free.saturating_sub(1);
                }
            }
        }
    }
    mem.insert(state, rocks.to_vec());
    (false, None)
}
