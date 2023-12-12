use std::{
    cell::RefCell,
    collections::HashMap,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Unknown,
    Working,
    Broken,
}

impl TryFrom<&u8> for Field {
    type Error = &'static str;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            b'?' => Ok(Self::Unknown),
            b'.' => Ok(Self::Working),
            b'#' => Ok(Self::Broken),
            _ => Err("Unknown Field"),
        }
    }
}

#[derive(Debug, Clone)]
struct Springs {
    springs: Vec<Field>,
    pattern: Vec<usize>,
}

impl Springs {
    fn new(springs: Vec<Field>, pattern: Vec<usize>) -> Self {
        Self { springs, pattern }
    }

    fn perms(&self) -> usize {
        Springs::count_perms(
            &self.springs,
            &self.pattern,
            Rc::new(RefCell::new(HashMap::new())),
        )
    }

    fn skip_working(springs: &[Field]) -> &[Field] {
        let s = springs
            .iter()
            .position(|f| *f != Field::Working)
            .unwrap_or(springs.len());
        &springs[s..]
    }

    fn calc_next(springs: &[Field], s: usize) -> Vec<&[Field]> {
        let mut next = Vec::new();
        for (i, window) in springs.windows(s + 1).enumerate() {
            if !window[..s].contains(&Field::Working) && window[s] != Field::Broken {
                next.push(Springs::skip_working(&springs[i + s + 1..]));
            }
            if window[0] == Field::Broken {
                break;
            }
        }
        next
    }

    fn count_perms(
        springs: &[Field],
        rest: &[usize],
        mem: Rc<RefCell<HashMap<(usize, usize), usize>>>,
    ) -> usize {
        let key = (springs.len(), rest.len());
        if let Some(m) = mem.borrow().get(&key) {
            return *m;
        }

        if let [current, rest @ ..] = rest {
            let r = Springs::calc_next(springs, *current)
                .iter()
                .map(|r| Springs::count_perms(r, rest, Rc::clone(&mem)))
                .sum();
            mem.borrow_mut().insert(key, r);
            r
        } else if springs.contains(&Field::Broken) {
            0
        } else {
            1
        }
    }
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut spring_vec = Vec::new();
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let (springs, pattern) = line
            .split_once(" ")
            .expect("Springs to be seperated by pattern with whitespace");
        let springs: Vec<Field> = springs
            .as_bytes()
            .into_iter()
            .chain(b"?")
            .chain(springs.as_bytes())
            .chain(b"?")
            .chain(springs.as_bytes())
            .chain(b"?")
            .chain(springs.as_bytes())
            .chain(b"?")
            .chain(springs.as_bytes())
            .chain(b".")
            .map(|b| b.try_into().unwrap())
            .collect();
        let pattern: Vec<usize> = pattern
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);
        spring_vec.push(Springs::new(springs, pattern))
    }
    let r = spring_vec.iter_mut().fold(0, |acc, s| acc + s.perms());
    println!("{r}")
}
