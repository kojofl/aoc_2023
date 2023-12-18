use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub instr: Direction,
    pub amount: u32,
    pub color: String,
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        let mut split = value.split_whitespace();
        let dir: Direction = split.next().unwrap().as_bytes()[0].into();
        let amount: u32 = split.next().unwrap().parse::<u32>().unwrap();
        let color = split
            .next()
            .unwrap()
            .strip_prefix("(#")
            .map(|s| s.strip_suffix(')').unwrap())
            .unwrap()
            .to_owned();
        Self {
            instr: dir,
            amount,
            color,
        }
    }
}

fn from_hex(s: String) -> Instruction {
    let (n, d) = s.split_at(5);
    let amount = u32::from_str_radix(n, 16).unwrap();
    let d = match d {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("unknown encoded direction"),
    };
    Instruction {
        instr: d,
        amount,
        color: s,
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'U' => Self::Up,
            b'R' => Self::Right,
            b'D' => Self::Down,
            b'L' => Self::Left,
            _ => panic!("Unable to parse byte to direction"),
        }
    }
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let mut instructions: Vec<Instruction> = BufReader::new(f)
        .lines()
        .map(|s| s.unwrap().into())
        .collect();
    let borders = dig(&instructions);
    let r = calc_ground_from_border(&borders);
    println!("{r}");
    // Part 2
    instructions.iter_mut().for_each(|i| {
        *i = from_hex(i.color.clone());
    });
    let borders = dig(&instructions);
    let r = calc_ground_from_border(&borders);
    println!("{r}");
}

fn calc_ground_from_border(border: &VecDeque<Vec<(Tile, usize)>>) -> usize {
    let mut ground_tiles = 0;
    for row in border {
        let fileter_row: Vec<(Tile, usize)> = row
            .iter()
            .copied()
            .filter(|(t, _)| *t != Tile::Horizontal)
            .collect();
        for (i, border) in fileter_row.iter().enumerate() {
            if i == 0 {
                ground_tiles += 1;
                continue;
            }
            match border.0 {
                Tile::Vertical => {
                    let mut inter = 0;
                    let mut dir: Option<Tile> = None;
                    for p in fileter_row[..i].iter() {
                        match p.0 {
                            Tile::Vertical => {
                                inter += 1;
                            }
                            Tile::CornerDown => match dir {
                                Some(t) => {
                                    if t == Tile::CornerUp {
                                        inter += 1;
                                    }
                                    dir = None;
                                }
                                None => {
                                    dir = Some(p.0);
                                }
                            },
                            Tile::CornerUp => match dir {
                                Some(t) => {
                                    if t == Tile::CornerDown {
                                        inter += 1;
                                    }
                                    dir = None;
                                }
                                None => {
                                    dir = Some(p.0);
                                }
                            },
                            _ => {}
                        }
                    }
                    if inter % 2 == 0 {
                        ground_tiles += 1;
                    } else {
                        ground_tiles += (fileter_row[i - 1].1..fileter_row[i].1).count();
                    }
                }
                Tile::CornerDown | Tile::CornerUp => {
                    let mut corners = 0;
                    let mut inter = 0;
                    let mut dir: Option<Tile> = None;
                    for p in fileter_row[..i].iter() {
                        match p.0 {
                            Tile::Vertical => {
                                inter += 1;
                            }
                            Tile::CornerDown => {
                                corners += 1;
                                match dir {
                                    Some(t) => {
                                        if t == Tile::CornerUp {
                                            inter += 1;
                                        }
                                        dir = None;
                                    }
                                    None => {
                                        dir = Some(p.0);
                                    }
                                }
                            }
                            Tile::CornerUp => {
                                corners += 1;
                                match dir {
                                    Some(t) => {
                                        if t == Tile::CornerDown {
                                            inter += 1;
                                        }
                                        dir = None;
                                    }
                                    None => {
                                        dir = Some(p.0);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    if corners % 2 != 0 {
                        ground_tiles += (fileter_row[i - 1].1..fileter_row[i].1).count();
                    } else if inter % 2 == 0 {
                        ground_tiles += 1;
                    } else {
                        ground_tiles += (fileter_row[i - 1].1..fileter_row[i].1).count();
                    }
                }
                _ => {}
            }
        }
    }
    ground_tiles
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    CornerDown,
    CornerUp,
}

fn dig(instructions: &[Instruction]) -> VecDeque<Vec<(Tile, usize)>> {
    let mut x: i64 = 0;
    let mut y: usize = 0;
    let mut borders: VecDeque<Vec<(Tile, i64)>> = VecDeque::new();
    borders.push_back(Vec::new());
    let start = &instructions[0].instr;
    let mut prev_dir = &instructions[0].instr;
    for i in instructions {
        match i.instr {
            Direction::Up => {
                match prev_dir {
                    Direction::Right | Direction::Left => match borders[y].last_mut() {
                        Some(v) => {
                            v.0 = Tile::CornerUp;
                        }
                        None => {
                            borders[y].push((Tile::CornerUp, x));
                        }
                    },
                    _ => {}
                }
                for _ in 0..i.amount {
                    let (new_y, is_overflow) = y.overflowing_sub(1);
                    if is_overflow {
                        borders.push_front(vec![(Tile::Vertical, x)]);
                    } else {
                        y = new_y;
                        borders[y].push((Tile::Vertical, x));
                    }
                }
            }
            Direction::Right => {
                match prev_dir {
                    Direction::Up => {
                        borders[y].last_mut().unwrap().0 = Tile::CornerDown;
                    }
                    Direction::Down => {
                        borders[y].last_mut().unwrap().0 = Tile::CornerUp;
                    }
                    _ => {}
                }
                x += i.amount as i64;
            }
            Direction::Down => {
                match prev_dir {
                    Direction::Right | Direction::Left => {
                        borders[y].last_mut().unwrap().0 = Tile::CornerDown
                    }
                    _ => {}
                }
                for _ in 0..i.amount {
                    y = y + 1;
                    if y >= borders.len() {
                        borders.push_back(vec![(Tile::Vertical, x)]);
                    } else {
                        borders[y].push((Tile::Vertical, x));
                    }
                }
            }
            Direction::Left => {
                match prev_dir {
                    Direction::Up => {
                        borders[y].last_mut().unwrap().0 = Tile::CornerDown;
                    }
                    Direction::Down => {
                        borders[y].last_mut().unwrap().0 = Tile::CornerUp;
                    }
                    _ => {}
                }
                x -= i.amount as i64;
            }
        }
        prev_dir = &i.instr;
    }
    match (&instructions.last().unwrap().instr, start) {
        (Direction::Up, Direction::Right)
        | (Direction::Up, Direction::Left)
        | (Direction::Right, Direction::Down)
        | (Direction::Left, Direction::Down) => {
            borders[y].last_mut().unwrap().0 = Tile::CornerDown;
        }
        (Direction::Right, Direction::Up)
        | (Direction::Down, Direction::Right)
        | (Direction::Down, Direction::Left)
        | (Direction::Left, Direction::Up) => {
            borders[y].last_mut().unwrap().0 = Tile::CornerUp;
        }
        _ => {}
    }
    let mut shift = 0;
    borders.iter_mut().for_each(|v| {
        v.sort_by(|a, b| a.1.cmp(&b.1));
        shift = shift.min(v[0].1);
    });
    shift = shift.abs();
    borders
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|(t, e)| (t, (e + shift) as usize))
                .collect()
        })
        .collect()
}
