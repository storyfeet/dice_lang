use crate::dice::Value;
use std::collections::BTreeMap;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Context {
    stack: Vec<Value>,
    rolls: Vec<Value>,
    labels: Vec<(String, Value)>,
    vars: BTreeMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            rolls: Vec::new(),
            labels: Vec::new(),
            vars: BTreeMap::new(),
        }
    }

    pub fn push(&mut self, dr: Value) {
        self.stack.push(dr);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn roll(&mut self, dr: Value) {
        self.rolls.push(dr);
    }
}

impl Display for Context {
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
