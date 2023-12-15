use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

struct Wasteland {
    pub instructions: String,
    pub starting: Vec<String>,
    pub map: HashMap<String, (String, String)>,
}

impl Wasteland {
    pub fn _run(&self) -> usize {
        let mut curr = self.map.get("AAA").unwrap();
        for (i, instr) in self.instructions.chars().cycle().enumerate() {
            match instr {
                'R' => {
                    if curr.1 == "ZZZ" {
                        return i + 1;
                    }
                    curr = self.map.get(&curr.1).unwrap();
                }
                'L' => {
                    if curr.0 == "ZZZ" {
                        return i + 1;
                    }
                    curr = self.map.get(&curr.0).unwrap();
                }
                _ => panic!("Unknown command"),
            }
        }
        0
    }

    pub fn run_ghost(&self) -> usize {
        let states: Vec<usize> = self
            .starting
            .iter()
            .map(|s| {
                let mut curr = self.map.get(s).unwrap();
                for (i, instr) in self.instructions.chars().cycle().enumerate() {
                    match instr {
                        'R' => {
                            if curr.1.ends_with('Z') {
                                return i + 1;
                            }
                            curr = self.map.get(&curr.1).unwrap();
                        }
                        'L' => {
                            if curr.0.ends_with('Z') {
                                return i + 1;
                            }
                            curr = self.map.get(&curr.0).unwrap();
                        }
                        _ => panic!("Unknown command"),
                    }
                }
                0
            })
            .collect();
        states.into_iter().fold(1, |acc, v| acc * v / gcd(acc, v))
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let file = std::fs::File::open("input");
    let reader = BufReader::new(file.unwrap());
    let mut lines = reader.lines();
    let instructions = lines.next().unwrap().unwrap();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut starts = Vec::new();
    for line in lines.skip(1) {
        let Ok(line) = line else {
            continue;
        };
        let (origin, dest) = line
            .split_once('=')
            .expect("source and dest to be seperated by '='");
        let origin = origin.trim();
        if origin.ends_with('A') {
            starts.push(origin.to_owned());
        }
        let mut s = dest.split_whitespace();
        let l = s.next().unwrap();
        let r = s.next().unwrap();
        map.insert(
            origin.into(),
            (
                unsafe { String::from_utf8_unchecked(l.as_bytes()[1..=3].to_vec()) },
                unsafe { String::from_utf8_unchecked(r.as_bytes()[..3].to_vec()) },
            ),
        );
    }
    let land = Wasteland {
        instructions,
        starting: starts,
        map,
    };
    let res = land.run_ghost();
    println!("{res}");
}
