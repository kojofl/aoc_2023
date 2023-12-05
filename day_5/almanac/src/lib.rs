pub mod range;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Almanac {
    pub states: [Vec<u64>; 8],
    pub translation: [Vec<Mapping>; 7],
}

impl Almanac {
    pub fn from_file(f: &str) -> Self {
        let file = std::fs::File::open(f).unwrap();
        let reader = BufReader::new(file);
        let mut states: [Vec<u64>; 8] = Default::default();
        let mut translation: [Vec<Mapping>; 7] = Default::default();
        let mut lines = reader.lines();
        states[0] = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let mut i = 0;
        let lines = lines.skip(2);
        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            if !line.as_bytes()[0].is_ascii_digit() {
                i += 1;
                continue;
            }
            let map_data = line
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let Some([dest, source_start, range]) = map_data.chunks_exact(3)
                .next() else {
                    panic!("Mappings need to consist of three numbers")
                };
            let m = Mapping {
                source: (*source_start, *source_start + *range - 1),
                dest: (*dest, *dest + *range - 1),
            };
            translation[i].push(m)
        }
        Self {
            states,
            translation,
        }
    }

    pub fn process_states(&mut self) {
        for i in 0..self.states.len() - 1 {
            let next: Vec<u64> = self.states[i]
                .iter()
                .map(|s| {
                    if let Some(m) = self.translation[i].iter().find(|t| t.source_contains(s)) {
                        s - m.source.0 + m.dest.0
                    } else {
                        *s
                    }
                })
                .collect();
            self.states[i + 1] = next
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    source: (u64, u64),
    dest: (u64, u64),
}

impl Mapping {
    pub fn source_contains(&self, el: &u64) -> bool {
        (self.source.0..=self.source.1).contains(el)
    }

    pub fn has_intersection(&self, el: &(u64, u64)) -> bool {
        !(el.1 < self.source.0 || el.0 > self.source.1)
    }
}
