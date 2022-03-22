use rand::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum Value {
    Num(i32),
    Word(String),
    Range(i32, i32),
    List(Vec<Value>),
}

impl Value {
    pub fn as_int(&self) -> anyhow::Result<i32> {
        match self {
            Self::Word(_) => err_tools::e_str("Cannot use String as Number"),
            Self::Num(n) => Ok(*n),
            Self::List(l) => l.iter().try_fold(0, |f, v| Ok(f + v.as_int()?)),
            Self::Range(_, _) => err_tools::e_str("Cannot use Range as Number"),
        }
    }

    pub fn flatten(&self) -> anyhow::Result<Vec<Value>> {
        match self {
            Self::List(l) => {
                let mut res = Vec::new();
                for i in l {
                    res.extend(i.flatten()?)
                }
                Ok(res)
            }
            v => Ok(vec![v.clone()]),
        }
    }

    pub fn roll_n(&self, n: i32) -> Value {
        let mut tr = rand::thread_rng();
        match n {
            1 => self.roll(&mut tr),
            v => {
                let mut res = Vec::new();
                for _ in 0..v {
                    res.push(self.roll(&mut tr));
                }
                Value::List(res)
            }
        }
    }

    pub fn roll<R: Rng>(&self, r: &mut R) -> Value {
        match self {
            Self::Num(10) => Value::Num(r.gen_range(0..10)),
            Self::Num(n) => Value::Num(r.gen_range(0..*n) + 1),
            Self::Range(a, b) if a < b => Value::Num(r.gen_range(*a..*b)),
            Self::Range(b, a) => Value::Num(r.gen_range(*a..*b)),
            Self::Word(s) => Value::Word(s.clone()),
            Self::List(v) => {
                let n = r.gen_range(0..v.len());
                v[n].clone()
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Word(s) => write!(f, "{}", s)?,
            Self::Num(n) => write!(f, "{}", n)?,
            Self::Range(b, t) => write!(f, "{}..{}", b, t)?,
            Self::List(l) => {
                let mut comma = "[";
                for i in l {
                    write!(f, "{}{}", comma, i)?;
                    comma = ", ";
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}
