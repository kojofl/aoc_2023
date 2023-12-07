use std::cmp::Ordering;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    J = 0,
    Numeric(u8),
    T = 10,
    Q = 12,
    K = 13,
    A = 14,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            b @ '2'..='9' => Card::Numeric(b.to_digit(10).unwrap() as u8),
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("Unknown card {value}"),
        }
    }
}

impl Card {
    pub fn as_value(&self) -> u8 {
        match self {
            Card::Numeric(v) => *v,
            _ => unsafe { *(self as *const Self as *const u8) },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Power {
    Five = 6,
    Four = 5,
    FullHouse = 4,
    Three = 3,
    TwoPair = 2,
    OnePair = 1,
    High = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
    pub power: Power,
    pub tie_pow: u64,
    pub bid: u64,
}

impl Hand {
    pub fn new(cards: &[Card; 5], bid: u64) -> Self {
        let mut count = [0; 13];
        let mut jokers = 0;
        let tie_pow = [
            cards[4].as_value(),
            cards[3].as_value(),
            cards[2].as_value(),
            cards[1].as_value(),
            cards[0].as_value(),
            0,
            0,
            0,
        ];
        for card in &tie_pow[..5] {
            match *card {
                0 => jokers += 1,
                _ => count[*card as usize - 2] += 1,
            }
        }
        count.sort_by(|a, b| b.cmp(&a));
        count[0] += jokers;
        let power = match &count[..2] {
            [5, ..] => Power::Five,
            [4, ..] => Power::Four,
            [3, 2] => Power::FullHouse,
            [3, ..] => Power::Three,
            [2, 2] => Power::TwoPair,
            [2, ..] => Power::OnePair,
            _ => Power::High,
        };
        Self {
            cards: *cards,
            power,
            tie_pow: unsafe { *(&tie_pow as *const [u8; 8] as *const u64) },
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let p_a = self.power as u8;
        let p_b = other.power as u8;
        match p_a.cmp(&p_b) {
            Ordering::Equal => Some(self.tie_pow.cmp(&other.tie_pow)),
            o @ _ => Some(o),
        }
    }
}
