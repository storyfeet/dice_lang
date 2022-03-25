use crate::context::Context;
use crate::dice::Value;
use err_tools::*;

#[derive(Clone, Debug)]
pub enum Operation {
    Num(i32),
    Word(String),
    List(i32), //Num elements
    Var,
    Add,
    Append,
    Sub,
    Neg,
    L,
    H,
    P,
    Fudge,
    D,
    Sum,
    Equal,
    Less,
    Greater,
    Range,
    Replace,
    Count,
    As,
}

macro_rules! job2 {
    ($ct:ident,$a:ident,$b:ident,$e:expr) => {{
        let $b = $ct.try_pop()?;
        let $a = $ct.try_pop()?;
        $ct.push($e);
    }};
}

impl Operation {
    pub fn resolve(&self, ct: &mut Context) -> anyhow::Result<()> {
        match self {
            Self::Add => job2!(ct, a, b, Value::Num(a.as_int()? + b.as_int()?)),
            Self::Append => job2!(ct, a, b, a.append(b)),
            Self::Neg => {
                let a = ct.try_pop()?;
                ct.push(Value::Num(-a.as_int()?));
            }
            Self::Sub => job2!(ct, a, b, Value::Num(a.as_int()? - b.as_int()?)),
            Self::Sum => {
                let a = ct.try_pop()?;
                ct.push(Value::Num(a.as_int()?));
            }
            Self::As => {
                let w = ct.try_pop()?.to_string();
                ct.push_var(w)?;
            }
            Self::L => {
                ct.push(ct.last_roll().e_str("No last roll for L")?.lowest()?);
            }
            Self::H => {
                ct.push(ct.last_roll().e_str("No last roll for H")?.highest()?);
            }
            Self::P => ct.push(ct.last_roll().e_str("No last roll for P")?),
            Self::Fudge => {
                ct.push(Value::List(vec![
                    Value::Num(-1),
                    Value::Num(0),
                    Value::Num(1),
                ]));
            }
            Self::Replace => {
                let v = ct.try_pop()?;
                ct.try_pop()?;
                ct.push(v);
            }
            Self::Equal => job2!(ct, a, b, a.filter(|v| *v == b)),
            Self::Less => job2!(ct, a, b, a.filter(|v| *v < b)),
            Self::Greater => job2!(ct, a, b, a.filter(|v| *v > b)),
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
                ct.var(&w)?;
            }
            Self::List(n) => {
                let l = ct.top_n(*n as usize)?;
                ct.push(Value::List(l));
            }
            Self::Count => {
                let l = ct.try_pop()?;
                ct.push(l.count());
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
        for o in &self.ops {
            o.resolve(ct)?;
        }
        ct.try_pop()
    }
}
