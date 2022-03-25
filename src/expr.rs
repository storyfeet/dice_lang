use crate::context::Context;
use crate::dice::Value;

#[derive(Clone, Debug)]
pub enum Operation {
    Num(i32),
    Word(String),
    List(i32), //Num elements
    Var,
    Value,
    Add,
    Sub,
    Neg,
    Push,
    Label,
    L,
    H,
    P,
    Fudge,
    Pop,
    D,
    Sum,
    Range,
    Replace,
}

impl Operation {
    pub fn resolve(&self, ct: &mut Context) -> anyhow::Result<()> {
        match self {
            Self::Add => {
                let b = ct.try_pop()?;
                let a = ct.try_pop()?;
                ct.push(Value::Num(a.as_int()? + b.as_int()?));
            }
            Self::Neg => {
                let a = ct.try_pop()?;
                ct.push(Value::Num(-a.as_int()?));
            }
            Self::Sub => {
                let b = ct.try_pop()?;
                let a = ct.try_pop()?;
                ct.push(Value::Num(a.as_int()? - b.as_int()?));
            }
            Self::Sum => {
                let a = ct.try_pop()?;
                ct.push(Value::Num(a.as_int()?));
            }
            Self::Label => {
                let a = ct.try_top()?;
                let w = ct.try_pop()?.to_string();
                ct.add_label(w, a);
            }
            Self::D => {
                let d = ct.try_pop()?;
                let n = ct.try_pop()?.as_int()?;
                //todo flatten
                let r = d.roll_n(n);
                ct.push_roll(r);
            }
            Self::Range => {
                let b = ct.try_pop()?.as_int()?;
                let a = ct.try_pop()?.as_int()?;
                ct.push(Value::Range(a, b));
            }
            Self::Num(n) => ct.push(Value::Num(*n)),
            Self::Word(s) => ct.push(Value::Word(s.clone())),
            Self::Var => {
                let w = ct.try_pop()?.to_string();
                ct.push_var(&w)?;
            }
            Self::List(n) => {
                let l = ct.top_n(*n as usize)?;
                ct.push(Value::List(l));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub ops: Vec<Operation>,
}

impl Expr {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }
    pub fn resolve(&self, ct: &mut Context) -> anyhow::Result<Value> {
        for o in self.ops {
            let mut res = o.resolve(ct)?;
        }
        ct.try_pop()
    }
}
