use std::{
    io::{BufRead, BufReader},
    slice::Iter,
};

type Field = Vec<Vec<u8>>;

#[derive(Clone, Debug)]
pub struct MirrorField {
    fields: Vec<Field>,
}

impl MirrorField {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn calc_mirror_sums(&self) -> usize {
        self.fields.iter().map(MirrorField::calc_mirror).sum()
    }

    pub fn calc_smudge_mirror_sums(&self) -> usize {
        self.fields
            .iter()
            .map(MirrorField::calc_smudge_mirror)
            .sum()
    }

    /// Checks for mirror placements that have no element off in the reflection.
    pub fn calc_mirror(field: &Field) -> usize {
        let mut possible_mirrors = Vec::new();
        for check in 1..field[0].len() {
            let (left, right) = field[0].split_at(check);
            let min = left.len().min(right.len());
            if right[0..min].iter().eq(left[check - min..].iter().rev()) {
                possible_mirrors.push(check);
            }
        }
        'c: for candidate in possible_mirrors.drain(0..possible_mirrors.len()) {
            for r in &field[1..] {
                let (left, right) = r.split_at(candidate);
                let min = left.len().min(right.len());
                if !right[0..min]
                    .iter()
                    .eq(left[candidate - min..].iter().rev())
                {
                    continue 'c;
                }
            }
            return candidate;
        }
        // No horizontal mirror found
        for (i, window) in field.windows(2).enumerate() {
            let [a, b] = window else {
                continue;
            };
            if a == b {
                possible_mirrors.push(i + 1)
            }
        }
        for candidate in possible_mirrors {
            let (top, bottom) = field.split_at(candidate);
            let min = top.len().min(bottom.len());
            if bottom[0..min]
                .iter()
                .eq(top[candidate - min..].iter().rev())
            {
                return candidate * 100;
            }
        }
        unreachable!()
    }

    /// Checks for mirror placements that have exactly one element off in the reflection.
    fn calc_smudge_mirror(field: &Field) -> usize {
        let mut possible_mirrors: Vec<(usize, bool)> = Vec::new();
        for check in 1..field[0].len() {
            let (left, right) = field[0].split_at(check);
            let min = left.len().min(right.len());
            if let Ok(r) = max_one_off(right[0..min].iter(), left[check - min..].iter(), true) {
                possible_mirrors.push((check, r));
            }
        }
        'c: for (candidate, mut d) in possible_mirrors.drain(0..possible_mirrors.len()) {
            for r in field[1..].iter() {
                let (left, right) = r.split_at(candidate);
                let min = left.len().min(right.len());
                if let Ok(r) =
                    max_one_off(right[0..min].iter(), left[candidate - min..].iter(), true)
                {
                    if r && d {
                        continue 'c;
                    }
                    d |= r;
                } else {
                    continue 'c;
                }
            }
            if d {
                return candidate;
            }
        }
        // No vertical mirror found
        for (i, window) in field.windows(2).enumerate() {
            let [a, b] = window else {
                continue;
            };
            if let Ok(r) = max_one_off(a.iter(), b.iter(), false) {
                possible_mirrors.push((i + 1, r))
            }
        }
        for (candidate, _d) in possible_mirrors.iter().copied() {
            let (top, bottom) = field.split_at(candidate);
            let min = top.len().min(bottom.len());
            match bottom[0..min]
                .iter()
                .zip(top[candidate - min..].iter().rev())
                .try_fold(0, |acc, (top, bottom)| {
                    if acc > 2 {
                        return Err(());
                    }
                    max_one_off(top.iter(), bottom.iter(), false).map(|e| acc + e as i32)
                }) {
                Ok(i) => {
                    if i == 0 {
                        continue;
                    }
                    return candidate * 100;
                }
                Err(_) => continue,
            }
        }
        unreachable!()
    }
}

/// Calculates if two u8 iterables are at most one element off.
/// If they differ in more elements returns Err.
/// On success returns if there was A element off.
fn max_one_off(a: Iter<'_, u8>, b: Iter<'_, u8>, reverse: bool) -> Result<bool, ()> {
    let mut one_off = false;
    if !reverse {
        for (a, b) in a.zip(b) {
            if a != b {
                match one_off {
                    true => return Err(()),
                    false => one_off = true,
                }
            }
        }
    } else {
        for (a, b) in a.zip(b.rev()) {
            if a != b {
                match one_off {
                    true => return Err(()),
                    false => one_off = true,
                }
            }
        }
    }
    Ok(one_off)
}

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let reader = BufReader::new(f);
    let mut fields = Vec::new();
    let mut field = Vec::new();
    for line in reader.lines().skip(1) {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            fields.push(field.clone());
            field.clear();
            continue;
        }
        field.push(String::into_bytes(line));
    }
    if !field.is_empty() {
        fields.push(field);
    }
    let mirror_field = MirrorField::new(fields);
    let r = mirror_field.calc_smudge_mirror_sums();
    println!("{r:?}")
}
