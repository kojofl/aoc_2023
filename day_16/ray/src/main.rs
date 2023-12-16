use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct Cave {
    pub inner: Vec<Vec<CaveElement>>,
    pub height: usize,
    pub width: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Top,
    Left,
    Right,
    Down,
}

type Position = (usize, usize);

pub enum CollisionResult {
    Split((Position, Direction), (Position, Direction)),
    Single((Position, Direction)),
}

#[derive(Clone, Copy, Debug)]
pub enum CaveElement {
    Empty,
    HorzontalMirror,
    VerticalMirror,
    LeftRightMirror,
    RightLeftMirror,
}

impl CaveElement {
    pub fn collision(&self, dir: Direction, position: Position) -> CollisionResult {
        match (self, dir) {
            (CaveElement::HorzontalMirror, Direction::Down)
            | (CaveElement::HorzontalMirror, Direction::Top) => CollisionResult::Split(
                (
                    (position.0, position.1.overflowing_sub(1).0),
                    Direction::Left,
                ),
                ((position.0, position.1 + 1), Direction::Right),
            ),
            (CaveElement::VerticalMirror, Direction::Left)
            | (CaveElement::VerticalMirror, Direction::Right) => CollisionResult::Split(
                ((position.0 + 1, position.1), Direction::Down),
                (
                    (position.0.overflowing_sub(1).0, position.1),
                    Direction::Top,
                ),
            ),
            (CaveElement::RightLeftMirror, Direction::Left)
            | (CaveElement::LeftRightMirror, Direction::Right) => {
                CollisionResult::Single(((position.0 + 1, position.1), Direction::Down))
            }
            (CaveElement::LeftRightMirror, Direction::Down)
            | (CaveElement::RightLeftMirror, Direction::Top) => {
                CollisionResult::Single(((position.0, position.1 + 1), Direction::Right))
            }
            (CaveElement::LeftRightMirror, Direction::Left)
            | (CaveElement::RightLeftMirror, Direction::Right) => CollisionResult::Single((
                (position.0.overflowing_sub(1).0, position.1),
                Direction::Top,
            )),
            (CaveElement::RightLeftMirror, Direction::Down)
            | (CaveElement::LeftRightMirror, Direction::Top) => CollisionResult::Single((
                (position.0, position.1.overflowing_sub(1).0),
                Direction::Left,
            )),
            _ => match dir {
                Direction::Top => {
                    CollisionResult::Single(((position.0.overflowing_sub(1).0, position.1), dir))
                }
                Direction::Left => {
                    CollisionResult::Single(((position.0, position.1.overflowing_sub(1).0), dir))
                }
                Direction::Right => CollisionResult::Single(((position.0, position.1 + 1), dir)),
                Direction::Down => CollisionResult::Single(((position.0 + 1, position.1), dir)),
            },
        }
    }
}

impl From<u8> for CaveElement {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'|' => Self::VerticalMirror,
            b'-' => Self::HorzontalMirror,
            b'/' => Self::RightLeftMirror,
            b'\\' => Self::LeftRightMirror,
            _ => panic!("Unknown cave element: {value}"),
        }
    }
}

impl Cave {
    pub fn new(cave: Vec<Vec<CaveElement>>) -> Self {
        let height = cave.len();
        let width = cave[0].len();
        Self {
            inner: cave,
            height,
            width,
        }
    }

    fn start_ray(&self) {
        let r = self.calc_ray_energy(Direction::Right, (0, 0));
        println!("{r}",);
    }

    fn start_rays(self: Arc<Self>) {
        let mut handles = Vec::new();
        for w in 0..self.width {
            let sel = Arc::clone(&self);
            handles.push(std::thread::spawn(move || {
                sel.calc_ray_energy(Direction::Down, (0, w))
            }));
            let sel = Arc::clone(&self);
            handles.push(std::thread::spawn(move || {
                sel.calc_ray_energy(Direction::Top, (sel.height - 1, w))
            }));
        }
        for h in 0..self.height {
            let sel = Arc::clone(&self);
            handles.push(std::thread::spawn(move || {
                sel.calc_ray_energy(Direction::Right, (h, 0))
            }));
            let sel = Arc::clone(&self);
            handles.push(std::thread::spawn(move || {
                sel.calc_ray_energy(Direction::Left, (h, sel.width - 1))
            }));
        }
        let r = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .max()
            .unwrap();
        println!("{r}")
    }

    fn calc_ray_energy(&self, direction: Direction, position: Position) -> u64 {
        let mut path: Vec<Vec<Vec<Direction>>> = vec![vec![Vec::new(); self.width]; self.height];
        let mut priority = VecDeque::new();
        priority.push_back((position, direction));

        while let Some((p, d)) = priority.pop_front() {
            if p.0 >= self.height || p.1 >= self.width {
                continue;
            }
            if path[p.0][p.1].contains(&d) {
                continue;
            }
            path[p.0][p.1].push(d);
            match self.inner[p.0][p.1].collision(d, p) {
                CollisionResult::Split((p_a, d_a), (p_b, d_b)) => {
                    priority.push_back((p_a, d_a));
                    priority.push_back((p_b, d_b));
                }
                CollisionResult::Single((p, d)) => priority.push_back((p, d)),
            }
        }
        path.iter().fold(0, |acc, p| {
            acc + (p.iter().fold(0, |acc, e| acc + (e.len() > 0) as u64))
        })
    }
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let cave: Vec<Vec<CaveElement>> = reader
        .lines()
        .map(|l| l.unwrap().bytes().map(CaveElement::from).collect())
        .collect();
    let cave = Cave::new(cave);
    cave.start_ray();
    let cave = Arc::new(cave);
    cave.start_rays();
}
