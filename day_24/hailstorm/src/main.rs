use std::io::{BufRead, BufReader};
use z3::ast::{Array, Ast, Int};
use z3::{Config, Context, Solver, Sort};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Hailstone {
    pos: DimensionContainter,
    vel: DimensionContainter,
    m: f64,
    b: f64,
}

impl Hailstone {
    pub fn new(pos: DimensionContainter, vel: DimensionContainter) -> Self {
        let m = vel.y / vel.x;
        let b = pos.y - pos.x * m;
        Self { pos, vel, m, b }
    }

    pub fn intersection(&self, other: &Hailstone) -> TwoDimIntersectionResult {
        //  y = x * m + b
        //  b = y - (xm)
        // Parallel
        if self.m == other.m {
            return TwoDimIntersectionResult::None;
        }
        let x = (other.b - self.b) / (self.m - other.m);
        let y = x * self.m + self.b;
        if self.vel.x > 0.0 && x < self.pos.x
            || self.vel.x < 0.0 && x > self.pos.x
            || other.vel.x > 0.0 && x < other.pos.x
            || other.vel.x < 0.0 && x > other.pos.x
            || self.vel.y > 0.0 && y < self.pos.y
            || self.vel.y < 0.0 && y > self.pos.y
            || other.vel.y > 0.0 && y < other.pos.y
            || other.vel.y < 0.0 && y > other.pos.y
        {
            return TwoDimIntersectionResult::Past { x, y };
        }
        TwoDimIntersectionResult::Intersection { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TwoDimIntersectionResult {
    Intersection { x: f64, y: f64 },
    Past { x: f64, y: f64 },
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DimensionContainter {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for DimensionContainter {
    fn from(value: &str) -> Self {
        let mut iter = value.split(',').map(str::trim);
        let x: f64 = iter.next().map(str::parse).unwrap().unwrap();
        let y: f64 = iter.next().map(str::parse).unwrap().unwrap();
        let z: f64 = iter.next().map(str::parse).unwrap().unwrap();
        Self { x, y, z }
    }
}

fn main() {
    let file = std::fs::File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let hailstones: Vec<Hailstone> = reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let pos = DimensionContainter::from(pos);
            let vel = DimensionContainter::from(vel);
            Hailstone::new(pos, vel)
        })
        .collect();
    let mut count = 0;
    let min = 200000000000000.0;
    let max = 400000000000000.0;
    for i in 0..hailstones.len() {
        let s = hailstones[i];
        for j in i + 1..hailstones.len() {
            let o = hailstones[j];
            if let TwoDimIntersectionResult::Intersection { x, y } = s.intersection(&o) {
                if (min..=max).contains(&x) && (min..=max).contains(&y) {
                    count += 1;
                }
            }
        }
    }
    println!("{count}");

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Z3 logic from arthomnix's part2: https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
    let [x, y, z, vx, vy, vz] = ["x", "y", "z", "vx", "vy", "vz"].map(|e| Int::new_const(&ctx, e));

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.pos.x as i64);
        let pyn = Int::from_i64(&ctx, hailstone.pos.y as i64);
        let pzn = Int::from_i64(&ctx, hailstone.pos.z as i64);
        let vxn = Int::from_i64(&ctx, hailstone.vel.x as i64);
        let vyn = Int::from_i64(&ctx, hailstone.vel.y as i64);
        let vzn = Int::from_i64(&ctx, hailstone.vel.z as i64);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&x + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&y + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&z + &vz * &tn)));
    }
    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();

    println!("{}", (x + y + z).to_string());
}
