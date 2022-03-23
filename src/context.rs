use crate::dice::Value;
use rand::rngs::ThreadRng;
use std::collections::BTreeMap;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Context {
    stack: Vec<Value>,
    rolls: Vec<Value>,
    labels: Vec<(String, Value)>,
    vars: BTreeMap<String, Value>,
    rng: ThreadRng,
}

impl Context {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            rolls: Vec::new(),
            labels: Vec::new(),
            vars: BTreeMap::new(),
            rng: rand::thread_rng(),
        }
    }

    pub fn prev(&self) -> Option<Value> {
        self.rolls.last().map(|c| c.clone())
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

    pub fn get_var(&mut self, s: &str) -> Option<Value> {
        self.vars.get(s).map(|v| v.clone())
    }

    pub fn add_label(&mut self, s: String, v: Value) {
        self.labels.push((s, v))
    }

    pub fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
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
