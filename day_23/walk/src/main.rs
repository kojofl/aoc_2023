use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Path,
    Forest,
    Slope(Direction),
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match value {
            b'#' => Field::Forest,
            b'.' => Field::Path,
            _ => Field::Slope(Direction::from(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            b'<' => Direction::Left,
            b'>' => Direction::Right,
            b'^' => Direction::Up,
            b'v' => Direction::Down,
            _ => panic!("Unknown dierection {value}"),
        }
    }
}

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(file);
    let maze: Vec<Vec<Field>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| String::into_bytes(s).into_iter().map(u8::into).collect())
        .collect();
    let r = run_maze(&maze);
    println!("{r}");
    let target_idx = (
        maze.len() - 1,
        maze.last()
            .unwrap()
            .iter()
            .position(|f| *f == Field::Path)
            .unwrap(),
    );
    let r = find_longest_path(
        &maze,
        (0, 1),
        target_idx,
        &mut vec![vec![false; maze[0].len()]; maze.len()],
    )
    .unwrap();
    println!("{r}");
}

fn run_maze(maze: &[Vec<Field>]) -> usize {
    let rows = maze.len();
    let cols = maze[0].len();
    let mut priority: VecDeque<((usize, usize), usize, HashSet<(usize, usize)>)> = VecDeque::new();
    // The start position is 0,1
    priority.push_back(((0, 1), 0, HashSet::new()));
    let mut target = 0;
    let target_idx = (
        rows - 1,
        maze.last()
            .unwrap()
            .iter()
            .position(|f| *f == Field::Path)
            .unwrap(),
    );
    while let Some(((i, j), s, mut seen)) = priority.pop_front() {
        seen.insert((i, j));
        if (i, j) == target_idx {
            target = target.max(s);
        }
        match maze[i][j] {
            Field::Path => {
                priority.extend(
                    [
                        ((i + 1, j), Direction::Up),
                        ((i.wrapping_sub(1), j), Direction::Down),
                        ((i, j + 1), Direction::Left),
                        ((i, j.wrapping_sub(1)), Direction::Right),
                    ]
                    .into_iter()
                    .filter(|(e, _)| e.0 < rows && e.1 < cols)
                    .filter(|((f_i, f_j), d)| {
                        maze[*f_i][*f_j] != Field::Forest && maze[*f_i][*f_j] != Field::Slope(*d)
                    })
                    .filter(|(f, _)| !seen.contains(&f))
                    .map(|(idx, _)| (idx, s + 1, seen.clone())),
                );
            }
            Field::Slope(d) => {
                let nex_idx = match d {
                    Direction::Up => (i - 1, j),
                    Direction::Down => (i + 1, j),
                    Direction::Left => (i, j - 1),
                    Direction::Right => (i, j + 1),
                };
                priority.push_back((nex_idx, s + 1, seen));
            }
            Field::Forest => unreachable!("Cannot walk through forest"),
        }
    }
    target
}

// Berry inefficient we should only visit conjunctions with respective weights to reduce recursion
// depth.
pub fn find_longest_path(
    maze: &[Vec<Field>],
    (i, j): (usize, usize),
    goal: (usize, usize),
    visited: &mut [Vec<bool>],
) -> Option<i64> {
    let rows = maze.len();
    let cols = maze[0].len();
    if (i, j) == goal {
        return Some(0);
    }
    if visited[i][j] {
        return None;
    }
    visited[i][j] = true;
    let mut res = i64::MIN;
    for next in [
        ((i + 1, j)),
        ((i.wrapping_sub(1), j)),
        ((i, j + 1)),
        ((i, j.wrapping_sub(1))),
    ]
    .into_iter()
    .filter(|e| e.0 < rows && e.1 < cols)
    .filter(|(f_i, f_j)| maze[*f_i][*f_j] != Field::Forest)
    {
        if let Some(r) = find_longest_path(maze, next, goal, visited) {
            res = res.max(r);
        }
    }
    visited[i][j] = false;
    if res == i64::MIN {
        None
    } else {
        Some(1 + res)
    }
}
