#[derive(Debug, Clone)]
pub struct Card {
    pub pulls: Vec<u32>,
    pub winning: Vec<u32>,
    pub amount: u32,
}

impl Card {
    pub fn new(mut pulls: Vec<u32>, winning: Vec<u32>) -> Self {
        pulls.sort();
        Self {
            pulls,
            winning,
            amount: 1,
        }
    }
    pub fn calc_winnings(&self) -> u32 {
        self.winning.iter().fold(0, |acc, wn| {
            if self.pulls.binary_search(&wn).is_err() {
                return acc;
            }
            if acc == 0 {
                1
            } else {
                acc * 2
            }
        })
    }
    pub fn calc_won_scratch(&self) -> u32 {
        self.winning.iter().fold(0, |acc, wn| {
            if self.pulls.binary_search(&wn).is_err() {
                return acc;
            }
            acc + 1
        })
    }
}

impl From<String> for Card {
    fn from(line: String) -> Self {
        let Some((pulls, winning)) = line.split(':').nth(1).expect("Each Card to have a ':'").split_once('|') else {
            panic!("Each card needs draws and winning numbers");
        };
        let pulls = pulls
            .trim()
            .split_whitespace()
            .map(|c| c.parse::<u32>().expect("Valid numbers"))
            .collect();
        let winning = winning
            .trim()
            .split_whitespace()
            .map(|c| c.parse::<u32>().expect("Valid numbers"))
            .collect();
        Card::new(pulls, winning)
    }
}
