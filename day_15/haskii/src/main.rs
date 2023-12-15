use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("input").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let s: usize = buffer
        .trim()
        .split(',')
        .map(|seq| hash(seq.as_bytes()))
        .sum();
    println!("{s}");

    let mut map: [Vec<String>; 256] = vec![Vec::new(); 256].try_into().unwrap();
    for cmd in buffer.trim().split(',').map(|s| Command::from(s)) {
        match cmd {
            Command::Insert { label, value } => {
                let idx = hash(label.as_bytes());
                match map[idx].iter().position(|s| s.starts_with(&label)) {
                    Some(j) => map[idx][j] = format!("{} {}", label, value),
                    None => map[idx].push(format!("{} {}", label, value)),
                }
            }
            Command::Remove { label } => {
                let idx = hash(label.as_bytes());
                match map[idx].iter().position(|s| s.starts_with(&label)) {
                    Some(j) => {
                        map[idx].remove(j);
                    }
                    None => {}
                }
            }
        }
    }
    let r = map.iter().enumerate().fold(0_usize, |acc, (i, bin)| {
        acc + bin.iter().enumerate().fold(0, |acc, (j, v)| {
            acc + (i + 1)
                * (j + 1)
                * v.split_ascii_whitespace()
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
        })
    });
    println!("{r}")
}

#[derive(Clone, Debug)]
pub enum Command {
    Insert { label: String, value: usize },
    Remove { label: String },
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        if let Some((label, value)) = value.split_once('=') {
            Self::Insert {
                label: label.to_string(),
                value: value.parse().unwrap(),
            }
        } else {
            Command::Remove {
                label: value[..value.len() - 1].to_string(),
            }
        }
    }
}

fn hash(seq: &[u8]) -> usize {
    let mut v = 0;
    for b in seq {
        v += *b as usize;
        v *= 17;
        v %= 256;
    }
    v
}
