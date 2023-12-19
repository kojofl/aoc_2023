use std::io::{BufRead, BufReader};

use nulzrulez::{Part, RulezNulz};

fn main() {
    let f = std::fs::File::open("input").unwrap();
    let mut lines = BufReader::new(f).lines();
    let id = RulezNulz::hash("in");
    let mut nulz_rulez = RulezNulz::new(id);
    for rule in lines
        .by_ref()
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| l.unwrap())
    {
        let (ident, rules) = rule
            .split_once('{')
            .map(|(a, b)| (RulezNulz::hash(a), b.strip_suffix('}').unwrap()))
            .unwrap();
        nulz_rulez.insert(ident, rules.into());
    }
    let mut r = 0;
    for accepted in lines
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .map(|s| Part::from(&s[..]))
        .filter(|p| nulz_rulez.is_accepted(*p))
    {
        r += accepted.sum_val();
    }
    println!("{r}");

    let r_2 = nulz_rulez
        .range_discovery()
        .into_iter()
        .fold(0, |acc, r| acc + r.prod_val());
    println!("{r_2}");
}
