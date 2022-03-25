use crate::dice::Value;
use err_tools::*;
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

    /// Push Var Value onto run stack
    pub fn push_var(&mut self, name: &str) -> anyhow::Result<()> {
        let v = self.vars.get(name).e_str("Could not get var")?.clone();
        self.stack.push(v);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn try_pop(&mut self) -> anyhow::Result<Value> {
        self.stack.pop().e_str("Nothing on Stack")
    }

    pub fn try_top(&mut self) -> anyhow::Result<Value> {
        self.stack
            .last()
            .e_str("Nothing on Stack")
            .map(Value::clone)
    }

    pub fn top_n(&mut self, n: usize) -> anyhow::Result<Vec<Value>> {
        let l = self.stack.len();
        if n > l {
            return e_str("Cannot take that many elements");
        }
        Ok(self.stack.split_off(l - n))
    }

    pub fn push_roll(&mut self, dr: Value) {
        self.stack.push(dr.clone());
        self.rolls.push(dr);
    }

    pub fn last_roll(&self) -> Option<Value> {
        self.rolls.last().map(Value::clone)
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
