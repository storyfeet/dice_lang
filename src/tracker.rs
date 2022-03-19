use crate::dice::DiceResult;

#[derive(Debug)]
pub struct Tracker {
    stack: Vec<DiceResult>,
    rolls: Vec<DiceResult>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            rolls: Vec::new(),
        }
    }

    pub fn push(&mut self, dr: DiceResult) {
        self.stack.push(dr);
    }

    pub fn pop(&mut self) -> Option<DiceResult> {
        self.stack.pop()
    }

    pub fn roll(&mut self, dr: DiceResult) {
        self.rolls.push(dr);
    }
}
