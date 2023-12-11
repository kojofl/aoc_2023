use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    sync::Arc,
};

struct Universe {
    /// The 'cost' of traveling to the tile horizontaly / vertically
    weights: Arc<Vec<Vec<(usize, usize)>>>,
    /// The galaxies in the universe
    galaxies: Arc<Vec<(usize, usize)>>,
}

impl Universe {
    fn new(weights: Vec<Vec<(usize, usize)>>, galaxies: Vec<(usize, usize)>) -> Self {
        Self {
            weights: Arc::new(weights),
            galaxies: Arc::new(galaxies),
        }
    }

    fn calculate_distance_sum(&self) -> usize {
        let mut handles = Vec::new();
        for galaxie in self.galaxies.iter() {
            let galaxie = galaxie.clone();
            let weights = Arc::clone(&self.weights);
            let galaxies = Arc::clone(&self.galaxies);
            handles.push(std::thread::spawn(move || {
                Universe::calc_dist(galaxie, weights, galaxies)
            }));
        }
        handles.into_iter().fold(0, |acc, handle| {
            acc + handle.join().unwrap().iter().sum::<usize>()
        }) / 2
    }

    fn start_state(
        start_row: usize,
        start_col: usize,
        rows: usize,
        cols: usize,
    ) -> (Vec<Vec<bool>>, Vec<Vec<usize>>) {
        let mut visited = vec![vec![false; cols]; rows];
        let mut dist = vec![vec![usize::MAX; cols]; rows];
        dist[start_row][start_col] = 0;
        visited[start_row][start_col] = true;
        (visited, dist)
    }

    fn calc_dist(
        (start_row, start_col): (usize, usize),
        weights: Arc<Vec<Vec<(usize, usize)>>>,
        galaxies: Arc<Vec<(usize, usize)>>,
    ) -> Vec<usize> {
        let rows = weights.len();
        let cols = weights[0].len();
        let (mut visited, mut dist) = Universe::start_state(start_row, start_col, rows, cols);
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let row_limit = rows - 1;
        let col_limit = cols - 1;
        match start_row {
            0 => {
                queue.push_back((start_row + 1, start_col));
                visited[start_row + 1][start_col] = true;
            }
            _ if start_row == row_limit => {
                queue.push_back((start_row - 1, start_col));
                visited[start_row - 1][start_col] = true;
            }
            _ if (1..row_limit).contains(&start_row) => {
                queue.push_back((start_row + 1, start_col));
                queue.push_back((start_row - 1, start_col));
                visited[start_row + 1][start_col] = true;
                visited[start_row - 1][start_col] = true;
            }
            _ => {}
        }
        match start_col {
            0 => {
                queue.push_back((start_row, start_col + 1));
                visited[start_row][start_col + 1] = true;
            }
            _ if start_col == col_limit => {
                queue.push_back((start_row, start_col - 1));
                visited[start_row][start_col - 1] = true;
            }
            _ if (1..col_limit).contains(&start_col) => {
                queue.push_back((start_row, start_col + 1));
                queue.push_back((start_row, start_col - 1));
                visited[start_row][start_col + 1] = true;
                visited[start_row][start_col - 1] = true;
            }
            _ => {}
        }
        while let Some((i, j)) = queue.pop_front() {
            dist[i][j] = [
                (i + 1, j),
                (i.checked_sub(1).unwrap_or(0), j),
                (i, j + 1),
                (i, j.checked_sub(1).unwrap_or(0)),
            ]
            .into_iter()
            .filter(|n| n.0 < rows && n.1 < cols)
            .filter(|n| dist[n.0][n.1] != usize::MAX && (n.0 != i || n.1 != j))
            .map(|n| {
                if n.0 == i {
                    dist[n.0][n.1] + weights[i][j].0
                } else {
                    dist[n.0][n.1] + weights[i][j].1
                }
            })
            .min()
            .unwrap();

            [
                (i + 1, j),
                (i.checked_sub(1).unwrap_or(0), j),
                (i, j + 1),
                (i, j.checked_sub(1).unwrap_or(0)),
            ]
            .into_iter()
            .filter(|n| n.0 < rows && n.1 < cols)
            .for_each(|n| {
                if !visited[n.0][n.1] {
                    queue.push_back(n);
                    visited[n.0][n.1] = true;
                }
            });
        }
        galaxies.iter().map(|n| dist[n.0][n.1]).collect()
    }
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut lines = reader.lines().peekable();
    let len = match lines.peek().unwrap() {
        Ok(s) => s.len(),
        Err(_) => 0,
    };
    let mut cols = vec![0; len];
    let mut weights = lines
        .map(|s| String::into_bytes(s.unwrap()))
        .enumerate()
        .map(|(i, b)| {
            let mut empty = true;
            for (j, byte) in b.iter().enumerate() {
                match byte {
                    b'#' => {
                        cols[j] += 1;
                        galaxies.push((i, j));
                        empty = false;
                    }
                    _ => {}
                }
            }
            let weight = if empty {
                vec![(1, 1000000); b.len()]
            } else {
                vec![(1, 1); b.len()]
            };
            weight
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
    for row in weights.iter_mut() {
        for (empty, _) in cols.iter().enumerate().filter(|(_, c)| **c == 0_i32) {
            row[empty].0 += 999999;
        }
    }
    let universe = Universe::new(weights, galaxies);
    let result = universe.calculate_distance_sum();
    println!("{result}")
}
