use crate::context::Context;
use crate::dice::Value;
use err_tools::*;

#[derive(Clone, Debug)]
// Values that require no prior item
pub enum ExValue {
    Num(i32),
    Word(String),
    Var(String),
    L,
    H,
    P,
    Fudge,
    Pop,
    List(Vec<Expr>),
    Neg(Box<Expr>),
    D(Box<Expr>),
    Ex(Box<Expr>),
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
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Value,
    Add,
    Sub,
    Neg,
    Push,
    Label,
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
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Expr {
    pub v: ExValue,
    pub op: Box<Operation>,
}

impl Expr {
    pub fn new(v: ExValue, op: Operation) -> Self {
        Self {
            v,
            op: Box::new(op),
        }
    }
    pub fn resolve(&self, c: &mut Context) -> anyhow::Result<Value> {
        let mut res = self.v.resolve(c)?;
        self.op.resolve(res, c)?;
        Ok(res)
    }
}
