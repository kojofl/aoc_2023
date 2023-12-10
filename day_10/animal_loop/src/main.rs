use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

fn main() {
    let f = std::fs::File::open("input");
    let reader = BufReader::new(f.unwrap());
    let mut map: Vec<Vec<u8>> = reader
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect();
    let p = map.iter().flatten().position(|e| *e == b'S').unwrap();
    let l = map[0].len();
    let (i, j) = (p / l, p % l);
    let mut hist = Vec::new();
    let r = run_maze((i, j), &mut map, &mut hist);
    println!("{r}");
    let mut b_map = vec![vec![Field::Inside; map[0].len()]; map.len()];
    for (i, j) in hist {
        b_map[i][j] = Field::Pipe(map[i][j]);
    }
    filter_non_contained(&mut b_map);
    ray_test(&mut b_map);
    let r = b_map
        .iter()
        .flatten()
        .filter(|e| match e {
            Field::Inside => true,
            _ => false,
        })
        .count();
    println!("{r}");
}

fn ray_test(b_map: &mut [Vec<Field>]) {
    for r in b_map.iter_mut() {
        for j in 0..r.len() {
            match r[j] {
                Field::Inside => {
                    let mut inter = 0;
                    let mut dir: Option<Direction> = None;
                    for p in r[..j].iter().filter(|e| match e {
                        Field::Pipe(_) => true,
                        _ => false,
                    }) {
                        if let Field::Pipe(b) = p {
                            match b {
                                b'|' => inter += 1,
                                b'L' | b'J' => match dir {
                                    Some(d) => match d {
                                        Direction::Bottom => {
                                            inter += 1;
                                            dir = None;
                                        }
                                        _ => dir = None,
                                    },
                                    None => {
                                        dir = Some(Direction::Top);
                                    }
                                },
                                b'7' | b'F' => match dir {
                                    Some(d) => match d {
                                        Direction::Top => {
                                            inter += 1;
                                            dir = None;
                                        }
                                        _ => dir = None,
                                    },
                                    None => dir = Some(Direction::Bottom),
                                },
                                _ => continue,
                            }
                        }
                    }
                    if inter % 2 == 0 {
                        r[j] = Field::Outside;
                    }
                }
                _ => continue,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Field {
    Outside,
    Inside,
    Pipe(u8),
}

fn filter_non_contained(b_map: &mut [Vec<Field>]) {
    while let Some(p) = b_map[0].iter().position(|e| match e {
        Field::Inside => true,
        _ => false,
    }) {
        proliferate((0, p), b_map);
    }
    while let Some(p) = b_map[b_map.len() - 1].iter().position(|e| match e {
        Field::Inside => true,
        _ => false,
    }) {
        proliferate((b_map.len() - 1, p), b_map);
    }
    while let Some(p) = b_map.iter().enumerate().find(|el| match el.1[0] {
        Field::Inside => true,
        _ => false,
    }) {
        proliferate((p.0, 0), b_map);
    }
    while let Some(p) = b_map
        .iter()
        .enumerate()
        .find(|el| match el.1[el.1.len() - 1] {
            Field::Inside => true,
            _ => false,
        })
    {
        proliferate((p.0, b_map[0].len() - 1), b_map);
    }
}

fn proliferate(seed: (usize, usize), b_map: &mut [Vec<Field>]) {
    let mut todo = VecDeque::new();
    todo.push_back(seed);
    while let Some((i, j)) = todo.pop_front() {
        match b_map[i][j] {
            Field::Inside => b_map[i][j] = Field::Outside,
            _ => continue,
        }
        if i > 0 {
            match b_map[i - 1][j] {
                Field::Inside => todo.push_back((i - 1, j)),
                _ => {}
            }
        }
        if j > 0 {
            match b_map[i][j - 1] {
                Field::Inside => todo.push_back((i, j - 1)),
                _ => {}
            }
        }
        if i < b_map.len() - 1 {
            match b_map[i + 1][j] {
                Field::Inside => todo.push_back((i + 1, j)),
                _ => {}
            }
        }
        if j < b_map[0].len() - 1 {
            match b_map[i][j + 1] {
                Field::Inside => todo.push_back((i, j + 1)),
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn run_maze(mut pos: (usize, usize), map: &mut [Vec<u8>], hist: &mut Vec<(usize, usize)>) -> u32 {
    let mut iteration = 1;
    let start = calc_start(&mut pos, &map);
    let mut dir = start[0];
    let (mut i, mut j) = (pos.0, pos.1);
    hist.push((i, j));

    while map[i][j] != b'S' {
        match map[i][j] {
            b'|' => match dir {
                Direction::Top => {
                    i += 1;
                }
                Direction::Bottom => {
                    i -= 1;
                }
                _ => unreachable!(),
            },
            b'-' => match dir {
                Direction::Left => {
                    j += 1;
                }
                Direction::Right => {
                    j -= 1;
                }
                _ => unreachable!(),
            },
            b'L' => match dir {
                Direction::Top => {
                    j += 1;
                    dir = Direction::Left;
                }
                Direction::Right => {
                    i -= 1;
                    dir = Direction::Bottom;
                }
                _ => unreachable!(),
            },
            b'J' => match dir {
                Direction::Top => {
                    j -= 1;
                    dir = Direction::Right;
                }
                Direction::Left => {
                    i -= 1;
                    dir = Direction::Bottom;
                }
                _ => todo!(),
            },
            b'7' => match dir {
                Direction::Bottom => {
                    j -= 1;
                    dir = Direction::Right;
                }
                Direction::Left => {
                    i += 1;
                    dir = Direction::Top;
                }
                _ => unreachable!(),
            },
            b'F' => match dir {
                Direction::Bottom => {
                    j += 1;
                    dir = Direction::Left;
                }
                Direction::Right => {
                    i += 1;
                    dir = Direction::Top;
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
        hist.push((i, j));
        iteration += 1;
    }

    map[i][j] = match start {
        [Direction::Top, Direction::Bottom] | [Direction::Bottom, Direction::Top] => b'|',
        [Direction::Top, Direction::Left] | [Direction::Left, Direction::Top] => b'F',
        [Direction::Top, Direction::Right] | [Direction::Right, Direction::Top] => b'7',
        [Direction::Bottom, Direction::Left] | [Direction::Left, Direction::Bottom] => b'L',
        [Direction::Bottom, Direction::Right] | [Direction::Right, Direction::Bottom] => b'J',
        [Direction::Right, Direction::Left] | [Direction::Left, Direction::Right] => b'-',
        _ => unreachable!(),
    };

    iteration / 2
}

fn calc_start((start_i, start_j): &mut (usize, usize), map: &[Vec<u8>]) -> [Direction; 2] {
    let mut directions = [Direction::Top; 2];
    let mut start_found = false;
    // Check top
    if [b'|', b'7', b'F'].contains(&map[start_i.checked_sub(1).unwrap_or(0)][*start_j]) {
        if !start_found {
            *start_i -= 1;
            directions[0] = Direction::Bottom;
            start_found = true;
        } else {
            directions[1] = Direction::Bottom;
        }
    }
    // Check bot
    if [b'|', b'J', b'L'].contains(&map[*start_i + 1][*start_j]) {
        if !start_found {
            *start_i += 1;
            directions[0] = Direction::Top;
            start_found = true;
        } else {
            directions[1] = Direction::Top;
        }
    }
    // Check left
    if [b'-', b'L', b'F'].contains(&map[*start_i][start_j.checked_sub(1).unwrap_or(0)]) {
        if !start_found {
            *start_j -= 1;
            directions[0] = Direction::Right;
        } else {
            directions[1] = Direction::Right;
        }
    }
    // Check right
    if [b'-', b'J', b'7'].contains(&map[*start_i][*start_j + 1]) {
        directions[1] = Direction::Left;
    }
    directions
}
