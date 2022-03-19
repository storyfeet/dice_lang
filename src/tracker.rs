use crate::dice::DiceResult;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Tracker {
    stack: Vec<DiceResult>,
    rolls: Vec<DiceResult>,
    labels: Vec<(String, DiceResult)>,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            rolls: Vec::new(),
            labels: Vec::new(),
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

impl Display for Tracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rolls : ")?;
        let mut comma = "";
        for r in &self.rolls {
            write!(f, "{}{}", comma, r)?;
            comma = "_ ";
        }

        if self.stack.len() > 0 {
            write!(f, "\nStack : ")?;
            let mut comma = "";
            for r in &self.stack {
                write!(f, "{}{}", comma, r)?;
                comma = "_ ";
            }
        }
        write!(f, "\n")?;

        for (l, r) in &self.labels {
            writeln!(f, "{} {}", l, r)?;
        }

        Ok(())
    }
}
