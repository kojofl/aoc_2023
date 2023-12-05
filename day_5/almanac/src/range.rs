use std::collections::VecDeque;

use crate::{Almanac, Mapping};

pub struct RangeAlmanac {
    pub states: [Vec<Range>; 8],
    pub translation: [Vec<Mapping>; 7],
}

impl From<Almanac> for RangeAlmanac {
    fn from(value: Almanac) -> Self {
        let mut states: [Vec<(u64, u64)>; 8] = Default::default();
        states[0] = value.states[0]
            .chunks_exact(2)
            .map(|e| (e[0], e[0] + e[1]))
            .collect();
        Self {
            states,
            translation: value.translation,
        }
    }
}

impl RangeAlmanac {
    pub fn map_data(&mut self) {
        for i in 0..self.states.len() - 1 {
            let mut list = VecDeque::new();
            let mut list_2 = VecDeque::new();
            let next: Vec<(u64, u64)> = self.states[i]
                .iter()
                .flat_map(|s| {
                    list.push_back(*s);
                    let mut temp = Vec::new();
                    for rule in self.translation[i].iter().filter(|t| t.has_intersection(s)) {
                        while let Some(el) = list.pop_front() {
                            if !rule.has_intersection(&el) {
                                list_2.push_back(el);
                                continue;
                            }
                            match intersect(el, *rule) {
                                IntersectionResult::Consumed(c) => temp.push(c),
                                IntersectionResult::Consumes(l, c, r) => {
                                    temp.push(c);
                                    list_2.push_back(l);
                                    list_2.push_back(r);
                                }
                                IntersectionResult::OverlapLeft(c, r) => {
                                    temp.push(c);
                                    list_2.push_back(r);
                                }
                                IntersectionResult::OverlapRight(r, c) => {
                                    temp.push(c);
                                    list_2.push_back(r);
                                }
                            }
                        }
                        std::mem::swap(&mut list, &mut list_2);
                    }
                    temp.extend(list.iter());
                    temp
                })
                .collect();
            self.states[i + 1] = next
        }
    }
}

type Range = (u64, u64);

pub enum IntersectionResult {
    Consumed(Range),
    Consumes(Range, Range, Range),
    // New state, rest
    OverlapLeft(Range, Range),
    // rest, New state
    OverlapRight(Range, Range),
}

pub fn intersect(input: Range, rule: Mapping) -> IntersectionResult {
    if input.0 >= rule.source.0 && input.1 <= rule.source.1 {
        IntersectionResult::Consumed((
            input.0 + rule.dest.0 - rule.source.0,
            input.1 + rule.dest.0 - rule.source.0,
        ))
    } else if input.0 >= rule.source.0 {
        IntersectionResult::OverlapLeft(
            (input.0 + rule.dest.0 - rule.source.0, rule.dest.1),
            (rule.source.1 + 1, input.1),
        )
    } else if input.1 <= rule.source.1 {
        IntersectionResult::OverlapRight(
            (input.0, rule.source.0 - 1),
            (rule.dest.0, input.1 - rule.source.0 + rule.dest.0),
        )
    } else {
        IntersectionResult::Consumes(
            (input.0, rule.source.0 - 1),
            rule.dest,
            (rule.source.1 + 1, input.1),
        )
    }
}
