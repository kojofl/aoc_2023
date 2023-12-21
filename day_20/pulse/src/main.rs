use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader},
};

#[derive(Clone, Debug)]
pub struct Machine {
    broadcast: Vec<usize>,
    inhabited: Vec<usize>,
    modules: Vec<Option<Module>>,
}

impl Machine {
    pub fn new(
        broadcast: Vec<usize>,
        inhabited: Vec<usize>,
        mut modules: Vec<Option<Module>>,
    ) -> Self {
        let conjunction_idx: Vec<usize> = inhabited
            .iter()
            .copied()
            .filter(|m| modules[*m].as_ref().unwrap().is_conjunction())
            .collect();
        for module_idx in inhabited.iter().copied() {
            let module = modules[module_idx].as_ref().unwrap();
            let next = match module {
                Module::FlipFlop { state: _, next } => next,
                Module::Conjunction { state: _, next } => next,
            }
            .clone();
            for n in next
                .iter()
                .copied()
                .filter(|idx| conjunction_idx.contains(idx))
            {
                let Some(Module::Conjunction { state, next: _ }) = modules[n].as_mut() else {
                    unreachable!()
                };
                state.insert(module_idx, Pulse::Low);
            }
        }
        Self {
            broadcast,
            inhabited,
            modules,
        }
    }

    pub fn init_pulse(&mut self, pulse: Pulse, iterations: usize) -> usize {
        use Pulse::*;
        let mut mem: HashSet<(usize, Pulse)> = HashSet::new();
        let mut total_pulses = (0, 0);
        let mut circle_start: Option<(usize, (usize, Pulse))> = None;
        let mut i = 0;
        let mut circle_pulses = (0, 0);
        while i < iterations {
            let mut pulse_counter = match pulse {
                High => (1, 0),
                Low => (0, 1),
            };
            let mut priority: VecDeque<(Option<usize>, usize, Pulse)> = VecDeque::new();
            let state = self.get_state();
            if let Some(_v) = mem.get(&(state, pulse)) {
                match circle_start.as_ref() {
                    Some(s) => {
                        if s.1 == (state, pulse) {
                            // break;
                        }
                    }
                    None => circle_start = Some((i, (state, pulse))),
                }
            }

            for i in self.broadcast.iter().copied() {
                match pulse {
                    High => {
                        pulse_counter.0 += 1;
                    }
                    Low => {
                        pulse_counter.1 += 1;
                    }
                };
                priority.push_back((None, i, pulse));
            }
            while let Some(c_m) = priority.pop_front() {
                if self.modules[c_m.1].is_none() {
                    continue;
                }
                match (
                    self.modules.get_mut(c_m.1).unwrap().as_mut().unwrap(),
                    c_m.2,
                ) {
                    (Module::FlipFlop { state, next }, Pulse::Low) => {
                        *state = !*state;
                        if *state {
                            pulse_counter.0 += next.len();
                            Self::send_pulse(c_m.1, High, next, &mut priority);
                        } else {
                            pulse_counter.1 += next.len();
                            Self::send_pulse(c_m.1, Low, next, &mut priority);
                        }
                    }
                    (Module::Conjunction { state, next }, Pulse::High) => {
                        state.insert(c_m.0.unwrap(), High);
                        if state.iter().all(|p| *p.1 == High) {
                            pulse_counter.1 += next.len();
                            Self::send_pulse(c_m.1, Low, next, &mut priority);
                        } else {
                            pulse_counter.0 += next.len();
                            Self::send_pulse(c_m.1, High, next, &mut priority);
                        }
                    }
                    (Module::Conjunction { state, next }, Pulse::Low) => {
                        state.insert(c_m.0.unwrap(), Low);
                        pulse_counter.0 += next.len();
                        Self::send_pulse(c_m.1, High, next, &mut priority);
                    }
                    _ => {}
                }
            }
            if circle_start.is_some() {
                circle_pulses.0 += pulse_counter.0;
                circle_pulses.1 += pulse_counter.1;
            }
            total_pulses.0 += pulse_counter.0;
            total_pulses.1 += pulse_counter.1;
            mem.insert((state, pulse));
            i += 1;
        }
        if let Some((c_s, _)) = circle_start.as_ref() {
            let circle_len = i - c_s;
            let remaining = iterations - i;
            let circle_iterations = remaining / circle_len;
            total_pulses.0 += circle_pulses.0 * circle_iterations;
            total_pulses.1 += circle_pulses.1 * circle_iterations;
            let reamining_pulses = remaining % circle_len;
            for _ in 0..reamining_pulses {
                let mut pulse_counter = match pulse {
                    High => (1, 0),
                    Low => (0, 1),
                };
                let mut priority: VecDeque<(Option<usize>, usize, Pulse)> = VecDeque::new();

                for i in self.broadcast.iter().copied() {
                    match pulse {
                        High => {
                            pulse_counter.0 += 1;
                        }
                        Low => {
                            pulse_counter.1 += 1;
                        }
                    };
                    priority.push_back((None, i, pulse));
                }
                while let Some(c_m) = priority.pop_front() {
                    match (
                        self.modules.get_mut(c_m.1).unwrap().as_mut().unwrap(),
                        c_m.2,
                    ) {
                        (Module::FlipFlop { state, next }, Pulse::Low) => {
                            *state = !*state;
                            if *state {
                                pulse_counter.0 += next.len();
                                Self::send_pulse(c_m.1, High, next, &mut priority);
                            } else {
                                pulse_counter.1 += next.len();
                                Self::send_pulse(c_m.1, Low, next, &mut priority);
                            }
                        }
                        (Module::Conjunction { state, next }, Pulse::High) => {
                            state.insert(c_m.0.unwrap(), High);
                            if state.iter().all(|p| *p.1 == High) {
                                pulse_counter.1 += next.len();
                                Self::send_pulse(c_m.1, Low, next, &mut priority);
                            }
                        }
                        (Module::Conjunction { state, next }, Pulse::Low) => {
                            state.insert(c_m.0.unwrap(), Low);
                            pulse_counter.0 += next.len();
                            Self::send_pulse(c_m.1, High, next, &mut priority);
                        }
                        _ => {}
                    }
                }
                total_pulses.0 += pulse_counter.0;
                total_pulses.1 += pulse_counter.1;
            }
        }

        total_pulses.0 * total_pulses.1
    }

    pub fn run_til_finish(&mut self) {
        use Pulse::*;
        // Hash of jm the predececor of rx a conjunction
        let goal = 246;
        let relevant: Vec<usize> = self
            .inhabited
            .iter()
            .copied()
            .filter(|idx| match self.modules[*idx].as_ref().unwrap() {
                Module::FlipFlop { state: _, next } => next.contains(&goal),
                Module::Conjunction { state: _, next } => next.contains(&goal),
            })
            .collect();
        let mut relevant_map: HashMap<usize, usize> = HashMap::new();

        for i in 1.. {
            let mut priority: VecDeque<(Option<usize>, usize, Pulse)> = VecDeque::new();

            for i in self.broadcast.iter().copied() {
                priority.push_back((None, i, Low));
            }
            while let Some(c_m) = priority.pop_front() {
                if let Some(sender) = c_m.0 {
                    if relevant.contains(&sender) && c_m.2 == High {
                        relevant_map.entry(sender).or_insert(i);
                        let r: Result<usize, &str> = relevant
                            .iter()
                            .map(|idx| relevant_map.get(idx))
                            .try_fold(1, |acc, v| {
                                let v = v.ok_or("not ready yet")?;
                                Ok(acc * v / gcd(acc, *v))
                            });
                        if let Ok(result) = r {
                            println!("{}", result);
                            return;
                        }
                    }
                }
                if self.modules[c_m.1].is_none() {
                    continue;
                }
                match (
                    self.modules.get_mut(c_m.1).unwrap().as_mut().unwrap(),
                    c_m.2,
                ) {
                    (Module::FlipFlop { state, next }, Pulse::Low) => {
                        *state = !*state;
                        if *state {
                            Self::send_pulse(c_m.1, High, next, &mut priority);
                        } else {
                            Self::send_pulse(c_m.1, Low, next, &mut priority);
                        }
                    }
                    (Module::Conjunction { state, next }, Pulse::High) => {
                        state.insert(c_m.0.unwrap(), High);
                        if state.iter().all(|p| *p.1 == High) {
                            Self::send_pulse(c_m.1, Low, next, &mut priority);
                        } else {
                            Self::send_pulse(c_m.1, High, next, &mut priority);
                        }
                    }
                    (Module::Conjunction { state, next }, Pulse::Low) => {
                        state.insert(c_m.0.unwrap(), Low);
                        Self::send_pulse(c_m.1, High, next, &mut priority);
                    }
                    _ => {}
                }
            }
        }
    }

    fn send_pulse(
        sender: usize,
        pulse: Pulse,
        targets: &[usize],
        list: &mut VecDeque<(Option<usize>, usize, Pulse)>,
    ) {
        for target in targets.iter() {
            list.push_back((Some(sender), *target, pulse))
        }
    }

    fn get_state(&self) -> usize {
        let mut state = 0;
        for i in &self.inhabited {
            match self.modules[*i].as_ref().unwrap() {
                Module::FlipFlop { state: s, next: _ } => {
                    state |= *s as usize;
                    state <<= 1;
                }
                Module::Conjunction { state: s, next: _ } => {
                    for i in self.inhabited.iter() {
                        if let Some(p) = s.get(i) {
                            match p {
                                Pulse::High => {
                                    state |= 1;
                                    state <<= 1;
                                }
                                Pulse::Low => {
                                    state <<= 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        state
    }
}

#[derive(Clone, Debug)]
pub enum Module {
    FlipFlop {
        state: bool,
        next: Vec<usize>,
    },
    Conjunction {
        state: HashMap<usize, Pulse>,
        next: Vec<usize>,
    },
}

impl Module {
    fn is_conjunction(&self) -> bool {
        match self {
            Module::FlipFlop { state: _, next: _ } => false,
            Module::Conjunction { state: _, next: _ } => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pulse {
    High,
    Low,
}

pub fn hash(input: &str) -> usize {
    let input = input.trim();
    assert!(input.len() <= 2, "{input}");

    let hash = input
        .as_bytes()
        .iter()
        .map(|b| {
            if !(97..=122).contains(b) {
                panic!("PerfectHash only supports lower character keys {input}");
            }
            b - 97
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + e as usize * 26_usize.pow(i as u32));
    hash
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let mut broadcast: Vec<usize> = Vec::new();
    let mut inhabited: Vec<usize> = Vec::new();
    let mut modules: Vec<Option<Module>> = vec![None; 676];
    for line in BufReader::new(f).lines().map(|l| l.unwrap()) {
        let (src, dest) = line.split_once("->").unwrap();
        match src.chars().next().unwrap() {
            'b' => {
                for dest in dest.split(", ") {
                    broadcast.push(hash(dest))
                }
            }
            '%' => {
                let idx = hash(&src[1..=2]);
                let next: Vec<usize> = dest.split(", ").map(hash).collect();
                inhabited.push(idx);
                modules[idx] = Some(Module::FlipFlop { state: false, next });
            }
            '&' => {
                let idx = hash(&src[1..=2]);
                let next: Vec<usize> = dest
                    .split(", ")
                    .filter(|s| !s.trim().is_empty())
                    .map(hash)
                    .collect();
                inhabited.push(idx);
                modules[idx] = Some(Module::Conjunction {
                    state: HashMap::new(),
                    next,
                });
            }
            _ => panic!("Unknown module type"),
        }
    }
    let mut machine = Machine::new(broadcast, inhabited, modules);
    machine.run_til_finish();
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[test]
fn test_hash() {
    let a = "sg";
    let b = "lm";
    let c = "dh";
    let d = "db";
    let a_hash = hash(a);
    let b_hash = hash(b);
    let c_hash = hash(c);
    let d_hash = hash(d);
    println!("{a_hash} - {b_hash} - {c_hash} - {d_hash}");
}
