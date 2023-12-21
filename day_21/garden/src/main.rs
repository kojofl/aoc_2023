use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GardenTile {
    Plot,
    Rock,
}

fn main() {
    let f = std::fs::File::open("sample").unwrap();
    let reader = BufReader::new(f);
    let mut field: Vec<Vec<GardenTile>> = Vec::new();
    let mut start = (0, 0);
    for (i, line) in reader.lines().enumerate() {
        let Ok(line) = line else {
            continue;
        };
        field.push(
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(j, b)| match b {
                    b'.' => GardenTile::Plot,
                    b'#' => GardenTile::Rock,
                    b'S' => {
                        start = (i, j);
                        GardenTile::Plot
                    }
                    _ => panic!("Unknown tile"),
                })
                .collect(),
        );
    }
    let r = run_maze_nulz(field, start, 5000);
    println!("{r}")
}

pub fn run_maze_nulz(
    maze: Vec<Vec<GardenTile>>,
    start: (usize, usize),
    step_limit: usize,
) -> usize {
    let mut priority = VecDeque::new();
    let rows = maze.len() as i64;
    let cols = maze[0].len() as i64;
    let mut counting = HashSet::new();
    let mut dont_need = HashSet::new();
    priority.push_back(((start.0 as i64, start.1 as i64), 0));
    while let Some(((i, j), s)) = priority.pop_front() {
        if counting.get(&(i, j)).is_some() || dont_need.get(&(i, j)).is_some() {
            continue;
        }
        if s % 2 == 0 {
            counting.insert((i, j));
        } else {
            dont_need.insert((i, j));
        }
        if s == step_limit {
            continue;
        }
        [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
            .into_iter()
            .for_each(|n| {
                if !counting.get(&n).is_some()
                    && !dont_need.get(&n).is_some()
                    && maze[(n.0.rem_euclid(rows)) as usize][(n.1.rem_euclid(cols)) as usize]
                        == GardenTile::Plot
                {
                    priority.push_back((n, s + 1));
                }
            });
    }
    counting.iter().count()
}
