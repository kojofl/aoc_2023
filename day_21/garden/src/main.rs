use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GardenTile {
    Plot,
    Rock,
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
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
    let r = run_maze_nulz(field, start, 26501365);
    println!("{r}")
}

pub fn run_maze_nulz(
    maze: Vec<Vec<GardenTile>>,
    start: (usize, usize),
    step_limit: usize,
) -> usize {
    let mut priority = VecDeque::new();
    let rows = maze.len();
    let cols = maze[0].len();
    let mut counting = vec![vec![usize::MAX; cols]; rows];
    priority.push_back(((start.0, start.1), 0));
    counting[start.0][start.1] = 0;
    let is_even = step_limit % 2 == 0;
    while let Some(((i, j), s)) = priority.pop_front() {
        if s == step_limit {
            continue;
        }
        [
            (i + 1, j),
            (i.wrapping_sub(1), j),
            (i, j + 1),
            (i, j.wrapping_sub(1)),
        ]
        .into_iter()
        .filter(|e| e.0 < rows && e.1 < cols)
        .for_each(|n| {
            if counting[n.0][n.1] == usize::MAX && maze[n.0][n.1] == GardenTile::Plot {
                counting[n.0][n.1] = s + 1;
                priority.push_back((n, s + 1));
            }
        });
    }
    let half = cols / 2;
    // Number of maps to consider
    let n = (step_limit.saturating_sub(half)) / cols;
    // The number of odd / even maps in n
    let (odds, evens) = if n % 2 == 0 {
        let odds = match is_even {
            true => n.pow(2),
            false => (n + 1).pow(2),
        };
        let evens = match is_even {
            true => (n + 1).pow(2),
            false => (n).pow(2),
        };
        (odds, evens)
    } else {
        let odds = match is_even {
            true => (n + 1).pow(2),
            false => n.pow(2),
        };
        let evens = match is_even {
            true => (n + 1).pow(2),
            false => n.pow(2),
        };
        (odds, evens)
    };
    // Sum of all odd tiles in odd maps in n
    let o = odds
        * counting
            .iter()
            .flatten()
            .copied()
            .filter(|s| *s != usize::MAX && s % 2 != 0)
            .count();
    // Sum of all even tiles in even maps in n
    let e = evens
        * counting
            .iter()
            .flatten()
            .copied()
            .filter(|s| *s != usize::MAX && s % 2 == 0)
            .count();
    let steps_last = step_limit % cols;
    // Since we cannot reach all tiles in the outermost maps we need to subtract them
    let outside = if is_even {
        (n + 1)
            * counting
                .iter()
                .flatten()
                .copied()
                .filter(|s| *s != usize::MAX && s % 2 == 0 && *s > steps_last)
                .count()
    } else {
        (n + 1)
            * counting
                .iter()
                .flatten()
                .copied()
                .filter(|s| *s != usize::MAX && s % 2 != 0 && *s > steps_last)
                .count()
    };
    // Since we do not consider some maps we need to consider their respective elements we can
    // reach
    let inside = if is_even {
        n * counting
            .iter()
            .flatten()
            .copied()
            .filter(|s| *s != usize::MAX && s % 2 != 0 && *s > steps_last)
            .count()
    } else {
        n * counting
            .iter()
            .flatten()
            .copied()
            .filter(|s| *s != usize::MAX && s % 2 == 0 && *s > steps_last)
            .count()
    };
    e + o - outside + inside
}
