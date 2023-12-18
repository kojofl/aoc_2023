use std::{
    io::{BufRead, BufReader},
    vec,
};

fn main() {
    let map: Vec<Vec<usize>> = BufReader::new(std::fs::File::open("input").expect("Nulz"))
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let r = run_maze(&map, 1, 3);
    println!("{:?}", r);
    let r = run_maze(&map, 4, 10);
    println!("{:?}", r);
}

#[derive(Clone, Copy, Debug)]
struct Field {
    position: (usize, usize),
    /// The direction the cart was moving
    dir: (Direction, u8),
    heat: usize,
}

type Position = (usize, usize);

impl Field {
    fn new(position: Position, dir: (Direction, u8), heat: usize) -> Self {
        Self {
            position,
            dir,
            heat,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    Top,
    Right,
    Left,
    Down,
    None,
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}

fn run_maze(map: &[Vec<usize>], min: u8, max: u8) -> Option<Field> {
    let height = map.len();
    let width = map[0].len();
    let mut visited = vec![vec![Vec::new(); width]; height];
    let mut priority = Vec::with_capacity(100);
    priority.push(Field::new((1, 0), (Direction::Down, 1), map[1][0] as usize));
    priority.push(Field::new((0, 1), (Direction::Right, 1), map[0][1]));
    priority.sort_by(|a, b| b.heat.cmp(&a.heat));
    while let Some(field) = priority.pop() {
        let i = field.position.0;
        let j = field.position.1;
        if i == height - 1 && j == width - 1 && field.dir.1 >= min {
            return Some(field);
        }
        if visited[i][j].contains(&field.dir) {
            continue;
        }
        visited[i][j].push(field.dir);
        match field.dir {
            (Direction::Down | Direction::Top, x) => {
                if x >= min {
                    let r = (i, j + 1);
                    let l = (i, j.wrapping_sub(1));
                    if r.1 < width {
                        if !visited[r.0][r.1].contains(&(Direction::Right, 1)) {
                            priority.push(Field::new(
                                r,
                                (Direction::Right, 1),
                                field.heat + map[r.0][r.1],
                            ));
                        }
                    }
                    if l.1 < width {
                        if !visited[l.0][l.1].contains(&(Direction::Left, 1)) {
                            priority.push(Field::new(
                                l,
                                (Direction::Left, 1),
                                field.heat + map[l.0][l.1],
                            ));
                        }
                    }
                }
                if x < max {
                    let d = if field.dir.0 == Direction::Down {
                        (i + 1, j)
                    } else {
                        (i.wrapping_sub(1), j)
                    };
                    if d.0 < height {
                        if !visited[d.0][d.1].contains(&(field.dir.0, x + 1)) {
                            priority.push(Field::new(
                                d,
                                (field.dir.0, x + 1),
                                field.heat + map[d.0][d.1],
                            ));
                        }
                    }
                }
            }
            (Direction::Left | Direction::Right, x) => {
                if x >= min {
                    let d = (i + 1, j);
                    let t = (i.wrapping_sub(1), j);
                    if d.0 < height {
                        if !visited[d.0][d.1].contains(&(Direction::Down, 1)) {
                            priority.push(Field::new(
                                d,
                                (Direction::Down, 1),
                                field.heat + map[d.0][d.1],
                            ));
                        }
                    }
                    if t.0 < height {
                        if !visited[t.0][t.1].contains(&(Direction::Top, 1)) {
                            priority.push(Field::new(
                                t,
                                (Direction::Top, 1),
                                field.heat + map[t.0][t.1],
                            ));
                        }
                    }
                }
                if x < max {
                    let l = if field.dir.0 == Direction::Left {
                        (i, j.wrapping_sub(1))
                    } else {
                        (i, j + 1)
                    };
                    if l.1 < width {
                        if !visited[l.0][l.1].contains(&(field.dir.0, x + 1)) {
                            priority.push(Field::new(
                                l,
                                (field.dir.0, x + 1),
                                field.heat + map[l.0][l.1],
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
        priority.sort_by(|a, b| b.heat.cmp(&a.heat));
    }
    None
}
