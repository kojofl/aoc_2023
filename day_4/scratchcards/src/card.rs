use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Card {
    pub pulls: HashSet<u32>,
    pub winning: HashSet<u32>,
    pub amount: u32,
}

impl Card {
    pub fn new(pulls: HashSet<u32>, winning: HashSet<u32>) -> Self {
        Self {
            pulls,
            winning,
            amount: 1,
        }
    }
    pub fn calc_winnings(&self) -> u32 {
        let wins = self.winning.intersection(&self.pulls).count() as u32;
        if wins == 0 {
            return 0;
        }
        2_u32.pow(wins - 1)
    }
    pub fn calc_won_scratch(&self) -> u32 {
        self.winning.intersection(&self.pulls).count() as u32
    }
}

impl From<String> for Card {
    fn from(line: String) -> Self {
        let Some((pulls, winning)) = line.split(':').nth(1).expect("Each Card to have a ':'").split_once('|') else {
            panic!("Each card needs draws and winning numbers");
        };
        let pulls = pulls
            .split_whitespace()
            .map(|c| c.parse::<u32>().expect("Valid numbers"))
            .collect();
        let winning = winning
            .split_whitespace()
            .map(|c| c.parse::<u32>().expect("Valid numbers"))
            .collect();
        Card::new(pulls, winning)
    }
}
