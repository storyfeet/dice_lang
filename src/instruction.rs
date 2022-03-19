use crate::dice::*;
use crate::tracker::Tracker;
use err_tools::*;

pub struct Job {
    d: Start,
    i: Vec<Instruction>,
}

impl Job {
    pub fn dice(d: NDice) -> Self {
        Self {
            d: Start::Roll(d),
            i: Vec::new(),
        }
    }
    pub fn pop() -> Self {
        Self {
            d: Start::Pop,
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

pub enum Start {
    Roll(NDice),
    Pop,
}

impl Start {
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
    //Add(i32),
    //Append(Job),
    Push,
}

impl Instruction {
    pub fn run(&self, dr: &DiceResult, stack: &mut Tracker) -> anyhow::Result<DiceResult> {
        match self {
            Self::Sum => {
                dr.0.iter()
                    .try_fold(0, |f, v| {
                        println!("f = {},v={:?}", f, v);
                        Ok(f + v.as_int()?)
                    })
                    .map(DiceResult::from)
            }
            Self::Push => {
                stack.push(dr.clone());
                Ok(dr.clone())
            }
        }
    }
}
