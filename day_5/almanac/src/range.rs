use std::collections::VecDeque;

use crate::{Almanac, Mapping};

type Range = (u64, u64);

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
    pub fn process_state(&mut self) {
        for i in 0..self.states.len() - 1 {
            // All elements that are being matched.
            let mut to_match = VecDeque::new();
            // Elements that have not matched are stored here for the next rule.
            let mut rest = VecDeque::new();
            let next_state: Vec<(u64, u64)> = self.states[i]
                .iter()
                .flat_map(|s| {
                    to_match.push_back(*s);
                    let mut temp = Vec::new();
                    for rule in self.translation[i].iter().filter(|t| t.has_intersection(s)) {
                        // Match all elements against rule
                        while let Some(el) = to_match.pop_front() {
                            if !rule.has_intersection(&el) {
                                rest.push_back(el);
                                continue;
                            }
                            match intersect(el, *rule) {
                                IntersectionResult::Consumed(c) => temp.push(c),
                                IntersectionResult::Consumes(l, c, r) => {
                                    temp.push(c);
                                    rest.push_back(l);
                                    rest.push_back(r);
                                }
                                IntersectionResult::OverlapLeft(c, r) => {
                                    temp.push(c);
                                    rest.push_back(r);
                                }
                                IntersectionResult::OverlapRight(r, c) => {
                                    temp.push(c);
                                    rest.push_back(r);
                                }
                            }
                        }
                        std::mem::swap(&mut to_match, &mut rest);
                    }
                    // Elements that never matched propagate to the next state
                    temp.extend(to_match.iter());
                    temp
                })
                .collect();
            self.states[i + 1] = next_state
        }
    }
}

// Result of matching a rule to an input
pub enum IntersectionResult {
    // new state
    Consumed(Range),
    // left, new state, right
    Consumes(Range, Range, Range),
    // New state, rest
    OverlapLeft(Range, Range),
    // rest, New state
    OverlapRight(Range, Range),
}

/// Calculates the IntersectionResult of a input with a matching rule.
/// Importantly the result of this function are nonsensical if the rule
/// does not match.
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
