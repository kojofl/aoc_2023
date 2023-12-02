#[derive(Debug, Clone, Copy, Default)]
pub struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn is_plausible(&self, input: &str) -> bool {
        match input.trim().split_once(' ').unwrap() {
            (n, "red") => n.parse::<u32>().expect("Valid number") <= self.red,
            (n, "blue") => n.parse::<u32>().expect("Valid number") <= self.blue,
            (n, "green") => n.parse::<u32>().expect("Valid number") <= self.green,
            _ => {
                panic!("Unknown color");
            }
        }
    }
    pub fn exchange_if_higher(&mut self, input: &str) {
        match input.trim().split_once(' ').unwrap() {
            (n, "red") => self.red = self.red.max(n.parse::<u32>().expect("Valid number")),
            (n, "blue") => self.blue = self.blue.max(n.parse::<u32>().expect("Valid number")),
            (n, "green") => self.green = self.green.max(n.parse::<u32>().expect("Valid number")),
            _ => {
                panic!("Unknown color");
            }
        }
    }
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}
