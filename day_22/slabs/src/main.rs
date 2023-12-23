use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
pub struct Cave {
    stones: Vec<Stone>,
    inner: Vec<Vec<Vec<usize>>>,
}

impl Cave {
    pub fn new(x: usize, y: usize, z: usize, mut stones: Vec<Stone>) -> Self {
        let mut cave = vec![vec![vec![0; x]; y]; z];
        Cave::let_stones_fall(&mut stones, &mut cave);
        Self {
            stones,
            inner: cave,
        }
    }
    /// Sorts stones by z index and then lets them come to rest in order.
    /// After coming to rest the cells in the cave will set to be occupied
    /// by the stone. The stone will reference the data in the cave it occupies.
    fn let_stones_fall(stones: &mut [Stone], cave: &mut [Vec<Vec<usize>>]) {
        stones.sort();
        for (i, stone) in stones.iter_mut().enumerate() {
            let i = i + 1;
            loop {
                if stone.z.0 == 1 {
                    break;
                }
                if cave[stone.z.0 - 1][stone.y.0..=stone.y.1]
                    .iter()
                    .map(|z| z[stone.x.0..=stone.x.1].iter())
                    .flatten()
                    .any(|v| *v != 0)
                {
                    break;
                }
                stone.z.0 -= 1;
                stone.z.1 -= 1;
            }

            for x in stone.x.0..=stone.x.1 {
                for y in stone.y.0..=stone.y.1 {
                    for z in stone.z.0..=stone.z.1 {
                        cave[z][y][x] = i;
                    }
                }
            }
        }
    }

    fn count_deletable_stones(&self) -> usize {
        self.stones.len()
            - self
                .stones
                .iter()
                .fold(HashSet::new(), |mut acc: HashSet<usize>, stone| {
                    let x: HashSet<usize> = self.inner[stone.z.0 - 1][stone.y.0..=stone.y.1]
                        .iter()
                        .map(|z| z[stone.x.0..=stone.x.1].iter())
                        .flatten()
                        .filter(|c| **c != 0)
                        .copied()
                        .collect();
                    if x.len() == 1 {
                        acc.extend(x.iter());
                    }
                    acc
                })
                .len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stone {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl Stone {
    pub fn new(x: (usize, usize), y: (usize, usize), z: (usize, usize)) -> Self {
        Self { x, y, z }
    }
}

impl Ord for Stone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Stone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.z.0.cmp(&other.z.0))
    }
}

fn main() {
    let file = std::fs::File::open("input").unwrap();
    let (input, max_x, max_y, max_z): (Vec<String>, usize, usize, usize) = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .fold((Vec::new(), 0, 0, 0), |mut acc, v| {
            let (_, max) = v.split_once('~').unwrap();
            let mut iter = max.split(',').map(|c| c.parse::<usize>().unwrap());
            acc.1 = acc.1.max(iter.next().unwrap() + 1);
            acc.2 = acc.2.max(iter.next().unwrap() + 1);
            acc.3 = acc.3.max(iter.next().unwrap() + 1);
            acc.0.push(v);
            acc
        });
    let mut stones: Vec<Stone> = Vec::new();
    for line in input.into_iter() {
        let (start, end) = line.split_once('~').unwrap();
        let start_values: [usize; 3] = start
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        let end_values: [usize; 3] = end
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        stones.push(Stone::new(
            (start_values[0], end_values[0]),
            (start_values[1], end_values[1]),
            (start_values[2], end_values[2]),
        ));
    }
    let cave = Cave::new(max_x, max_y, max_z, stones);
    let r = cave.count_deletable_stones();
    println!("{r}");
}
