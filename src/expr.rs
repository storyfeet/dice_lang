use crate::context::Context;
use crate::dice::Value;
use err_tools::*;

#[derive(Clone, Debug)]
pub enum ExValue {
    Num(i32),
    Word(String),
    Var(String),
    L,
    H,
    P,
    Fudge,
    Pop,
    List(Vec<ExValue>),
    Ex(Expr),
}

impl ExValue {
    pub fn resolve(&self, c: &mut Context) -> anyhow::Result<Value> {
        match self {
            Self::Num(n) => Ok(Value::Num(*n)),
            Self::Word(s) => Ok(Value::Word(s.clone())),
            Self::Var(v) => Ok(c.get_var(v).e_str("Var Does not Exist")?),
            Self::Pop => Ok(c.pop().e_str("Nothing to Pop")?),
            Self::P => Ok(c.prev().e_str("No Previous Roll")?),
            Self::H => c.prev().e_str("No Previous Roll")?.highest(),
            Self::L => c.prev().e_str("No Previous Roll")?.lowest(),
            Self::Fudge => Ok(Value::Range(-1, 2)),
            Self::List(l) => {
                let mut res = Vec::new();
                for i in l {
                    //TODO flatten ranges etc
                    res.push(i.resolve(c)?);
                }
                Ok(Value::List(res))
            }
            Self::Ex(e) => e.resolve(c),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Add(ExValue),
    Sub(ExValue),
    Push,
    Label(ExValue),
    D(ExValue),
    Sum,
    Range(ExValue),
    Replace(ExValue),
}

impl Operation {
    pub fn resolve(&self, a: Value, c: &mut Context) -> anyhow::Result<Value> {
        match self {
            Self::Add(b) => {
                let b = b.resolve(c)?;
                Ok(Value::Num(a.as_int()? + b.as_int()?))
            }
            Self::Sub(b) => {
                let b = b.resolve(c)?;
                Ok(Value::Num(a.as_int()? - b.as_int()?))
            }
            Self::Sum => Ok(Value::Num(a.as_int()?)),
            Self::Push => {
                c.push(a.clone());
                Ok(a)
            }
            Self::Label(l) => {
                let w = l.resolve(c)?.to_string();
                c.add_label(w, a.clone());
                Ok(a)
            }
            Self::D(e) => {
                let n = a.as_int()?;
                let e = e.resolve(c)?;
                //todo flatten
                let r = e.roll_n(n);
                c.roll(r.clone());
                Ok(r)
            }
            Self::Range(b) => {
                let a = a.as_int()?;
                let b = b.resolve(c)?.as_int()?;
                Ok(Value::Range(a, b))
            }
            Self::Replace(b) => b.resolve(c),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub v: Box<ExValue>,
    pub ops: Vec<Operation>,
}

impl Expr {
    pub fn resolve(&self, c: &mut Context) -> anyhow::Result<Value> {
        let mut res = self.v.resolve(c)?;
        for o in &self.ops {
            res = o.resolve(res, c)?;
        }
        Ok(res)
    }
}
