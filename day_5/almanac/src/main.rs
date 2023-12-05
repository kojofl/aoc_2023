use almanac::{range::RangeAlmanac, Almanac};

fn main() {
    let mut almanac = Almanac::from_file("./input");
    almanac.map_data();
    let mut almac_range: RangeAlmanac = almanac.into();
    almac_range.map_data();
    let min = almac_range
        .states
        .last()
        .unwrap()
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap();
    println!("{}", min.0)
}
