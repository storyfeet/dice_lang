use err_tools::*;
use rand::*;
use std::cmp::{Ordering, PartialOrd};
use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Num(i32),
    Word(String),
    Range(i32, i32),
    List(Vec<Value>),
}

impl Value {
    pub fn as_int(&self) -> anyhow::Result<i32> {
        match self {
            Self::Word(_) => e_str("Cannot use String as Number"),
            Self::Num(n) => Ok(*n),
            Self::List(l) => l.iter().try_fold(0, |f, v| Ok(f + v.as_int()?)),
            Self::Range(_, _) => e_str("Cannot use Range as Number"),
        }
    }

    fn _most<F: Fn(i32, i32) -> i32>(&self, f: F) -> anyhow::Result<i32> {
        match self {
            Self::Word(_) => e_str("Words are not High or Low"),
            Self::Num(n) => Ok(*n),
            Self::Range(a, b) => Ok((*a).max(*b)),
            Self::List(l) => {
                let mut res = None;
                for i in l {
                    if let Self::Num(n) = i {
                        match res {
                            None => res = Some(*n),
                            Some(p) => res = Some(f(p, *n)),
                        }
                    }
                }
                res.e_str("No Numbers in value")
            }
        }
    }

    pub fn filter<F: Fn(&Value) -> bool>(&self, f: F) -> Value {
        match self {
            Value::List(l) => Value::List(l.iter().filter(|v| f(v)).map(Value::clone).collect()),
            v => match f(v) {
                true => v.clone(),
                false => Value::List(Vec::new()),
            },
        }
    }
    pub fn highest(&self) -> anyhow::Result<Value> {
        self._most(i32::max).map(Value::Num)
    }
    pub fn lowest(&self) -> anyhow::Result<Value> {
        self._most(i32::min).map(Value::Num)
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
                if v.len() == 0 {
                    return Value::Num(0);
                }
                let n = r.gen_range(0..v.len());
                v[n].clone()
            }
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        use Value::*;
        match (self, other) {
            (Num(a), Num(b)) => a.cmp(b),
            (Word(a), Word(b)) => a.cmp(b),
            (Range(al, ah), Range(bl, bh)) => (2 * ah - al).cmp(&(2 * bh - bl)),
            (List(a), List(b)) => a.cmp(b),
            (Num(_), _) => Ordering::Less,
            (_, Num(_)) => Ordering::Greater,
            (Word(_), _) => Ordering::Less,
            (_, Word(_)) => Ordering::Greater,
            (Range(_, _), _) => Ordering::Less,
            (_, Range(_, _)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
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
