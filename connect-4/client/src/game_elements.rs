pub mod nav_bar;

#[derive(PartialEq, Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

pub enum Turn {
    First,
    Second,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::First => *self = Turn::Second,
            Turn::Second => *self = Turn::First,
        }
    }
}