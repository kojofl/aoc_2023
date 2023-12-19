use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct RulezNulz {
    init: usize,
    /// A perfect hash table for 3 Byte strings
    buffer: Vec<Option<RuleContainer>>,
}

impl RulezNulz {
    pub fn new(init: usize) -> Self {
        RulezNulz {
            init,
            buffer: vec![None; 18278],
        }
    }

    pub fn insert(&mut self, hash: usize, val: RuleContainer) {
        if self.buffer[hash].is_some() {
            println!("Overriding rule at {hash}");
        }
        self.buffer[hash] = Some(val);
    }

    pub fn get(&self, hash: usize) -> &RuleContainer {
        self.buffer[hash].as_ref().unwrap()
    }

    pub fn is_accepted(&self, p: Part) -> bool {
        let mut rule = self.buffer[self.init].as_ref().unwrap();
        let mut hash_set = HashSet::new();
        hash_set.insert(self.init);
        loop {
            let r = rule.match_against(p);
            match r {
                RuleResult::Next(idx) => {
                    if !hash_set.insert(idx) {
                        panic!("Running in circle");
                    }
                    rule = self.buffer[idx].as_ref().unwrap();
                }
                RuleResult::Accept => return true,
                RuleResult::Reject => return false,
            }
        }
    }

    pub fn hash(key: &str) -> usize {
        assert!(key.len() <= 3);
        key.as_bytes()
            .iter()
            .map(|b| {
                if !(97..=122).contains(b) {
                    panic!("PerfectHash only supports lower character keys");
                }
                b - 96
            })
            .rev()
            .enumerate()
            .fold(0, |acc, (i, e)| acc + e as usize * 26_usize.pow(i as u32))
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuleContainer {
    rules: Vec<Rule>,
    finaly: RuleResult,
}

impl RuleContainer {
    fn match_against(&self, p: Part) -> RuleResult {
        for rule in &self.rules {
            if let Some(r) = rule.matches(p) {
                return r;
            }
        }
        self.finaly
    }
}

impl From<&str> for RuleContainer {
    fn from(value: &str) -> Self {
        let mut v: Vec<Rule> = Vec::new();
        for rule in value.split(',') {
            if rule.chars().any(|c| c == '<' || c == '>') {
                v.push(rule.into());
            } else {
                let finaly = match rule {
                    "A" => RuleResult::Accept,
                    "R" => RuleResult::Reject,
                    _ => RuleResult::Next(RulezNulz::hash(rule)),
                };
                return Self { rules: v, finaly };
            }
        }
        panic!("Needs a final instruction")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut v: [Option<u32>; 4] = [None; 4];
        let striped = value
            .strip_prefix("{")
            .map(|s| s.strip_suffix("}").unwrap())
            .unwrap();
        for val in striped.split(',') {
            match &val.chars().nth(0).unwrap() {
                'x' => v[0] = Some(val[2..].parse().unwrap()),
                'm' => v[1] = Some(val[2..].parse().unwrap()),
                'a' => v[2] = Some(val[2..].parse().unwrap()),
                's' => v[3] = Some(val[2..].parse().unwrap()),
                _ => panic!("Unknown part value"),
            }
        }
        Self {
            x: v[0].unwrap(),
            m: v[1].unwrap(),
            a: v[2].unwrap(),
            s: v[3].unwrap(),
        }
    }
}

impl Part {
    pub fn sum_val(&self) -> u32 {
        self.s + self.x + self.a + self.m
    }

    fn get_part_value(&self, v: PartValue) -> u32 {
        match v {
            PartValue::X => self.x,
            PartValue::M => self.m,
            PartValue::S => self.s,
            PartValue::A => self.a,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum PartValue {
    X,
    M,
    S,
    A,
}

#[derive(Debug, Clone)]
pub struct Rule {
    val: PartValue,
    op: Operation,
    cmp: u32,
    result: RuleResult,
}

impl Rule {
    fn matches(&self, p: Part) -> Option<RuleResult> {
        let v = p.get_part_value(self.val);
        match self.op {
            Operation::Greater => {
                if v > self.cmp {
                    return Some(self.result);
                }
            }
            Operation::Smaller => {
                if v < self.cmp {
                    return Some(self.result);
                }
            }
        }
        None
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (o, o_i, c): (Option<Operation>, Option<usize>, Option<usize>) = value
            .as_bytes()
            .iter()
            .enumerate()
            .fold((None, None, None), |mut acc, (i, b)| {
                match b {
                    b'<' => {
                        assert!(acc.0.is_none());
                        acc.0 = Some(Operation::Smaller);
                        acc.1 = Some(i);
                    }
                    b'>' => {
                        assert!(acc.1.is_none());
                        acc.0 = Some(Operation::Greater);
                        acc.1 = Some(i);
                    }
                    b':' => {
                        assert!(acc.2.is_none());
                        acc.2 = Some(i);
                    }
                    _ => {}
                }
                acc
            });
        let op = o.unwrap();
        let o_i = o_i.unwrap();
        let val = match &value[..o_i] {
            "a" => PartValue::A,
            "x" => PartValue::X,
            "m" => PartValue::M,
            "s" => PartValue::S,
            _ => panic!("Unknown part"),
        };
        let c = c.unwrap();
        let cmp = value[o_i + 1..c].parse::<u32>().unwrap();
        let result = match &value[c + 1..] {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            _ => RuleResult::Next(RulezNulz::hash(&value[c + 1..])),
        };
        Self {
            val,
            op,
            cmp,
            result,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum RuleResult {
    Next(usize),
    #[default]
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Greater,
    Smaller,
}

#[test]
fn test_hash() {
    let t = "zzz";
    println!("{}", RulezNulz::hash(&t));
    let t = "z";
    println!("{}", RulezNulz::hash(&t));
}
