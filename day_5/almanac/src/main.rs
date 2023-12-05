use almanac::{range::RangeAlmanac, Almanac};

fn main() {
    // Part 1:
    let mut almanac = Almanac::from_file("./input");
    almanac.process_states();
    // Part 2:
    let mut almac_range: RangeAlmanac = almanac.into();
    almac_range.process_state();
    let min = almac_range
        .states
        .last()
        .unwrap()
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    println!("{}", min.0)
}
