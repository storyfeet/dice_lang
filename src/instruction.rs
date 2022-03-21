use crate::dice::*;
use crate::tracker::Tracker;
use err_tools::*;

pub struct Job {
    d: Value,
    i: Vec<Instruction>,
}

impl Job {
    pub fn dice(d: NDice) -> Self {
        Self {
            d: Value::Roll(d),
            i: Vec::new(),
        }
    }
    pub fn pop() -> Self {
        Self {
            d: Value::Pop,
            i: Vec::new(),
        }
    }
    pub fn num(n: i32) -> Self {
        Self {
            d: Value::Num(n),
            i: Vec::new(),
        }
    }

    pub fn run(&self, stack: &mut Tracker) -> anyhow::Result<DiceResult> {
        let mut dr = self.d.run(stack)?;
        for i in &self.i {
            dr = i.run(&dr, stack)?;
        }
        Ok(dr)
    }

    pub fn push_ins(mut self, i: Instruction) -> Self {
        self.i.push(i);
        self
    }
}

pub enum Value {
    Roll(NDice),
    Pop,
    Num(i32),
    Job(Box<Job>),
}

impl Value {
    pub fn run(&self, stack: &mut Tracker) -> anyhow::Result<DiceResult> {
        match self {
            Self::Roll(d) => {
                let v = d.roll();
                stack.roll(v.clone());
                Ok(v)
            }
            Self::Pop => stack.pop().e_str("Nothing to Pop"),
        }
    }
}

pub enum Instruction {
    //Map(Box<Instruction>),
    Sum,
    Add(Value),
    Append(Job),
    Push,
}

impl Instruction {
    pub fn run(&self, dr: &DiceResult, stack: &mut Tracker) -> anyhow::Result<DiceResult> {
        match self {
            Self::Sum => {
                dr.0.iter()
                    .try_fold(0, |f, v| Ok(f + v.as_int()?))
                    .map(DiceResult::from)
            }
            Self::Push => {
                stack.push(dr.clone());
                Ok(dr.clone())
            }
            Self::Append(j) => {
                let jr = j.run(stack)?;
                let mut d2 = dr.clone();
                d2.0.extend(jr.0);
                Ok(d2)
            }
        }
    }
}
