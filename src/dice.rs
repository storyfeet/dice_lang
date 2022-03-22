use crate::context::Context;
use rand::*;
use std::fmt::{self, Display};

#[derive(Clone, Debug)]
pub enum Value {
    Num(i32),
    Word(String),
    Range(i32, i32),
    List(Vec<Value>),
}

#[derive(Clone, Debug)]
pub enum PreValue {
    Simple(Value),
    Var(String),
    Pop,
    List(Vec<PreValue>),
    Range(Box<PreValue>, Box<PreValue>),
    Roll(Box<PreValue>, Box<PreValue>),
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

    pub fn flatten(&self)->anyhow::Result<Vec<Value>>{
        match self{
            Self::List(l)=>{
                let res = Vec::new();
                for i in l {
                    res.extend(i.flatten()?)    
                }
                Ok(res)
            }
            v=>Ok(vec![v.clone()])
        }
    }

    pub fn roll_n(&self,n:i32)->Value{
        let mut tr = rand::thread_rng();
        match n{
            1=> self.roll(&mut tr),
            v=>{
                let res = Vec::new();
                for r in 0..n{
                    res.push(self.roll(&mut tr));
                }
                Value::List(res)
            }
        }


    }

    pub fn roll<R:Rng>(&self,r:&mut R)->Value{
        match self {
            Self::Num(10) => Value::Num(r.gen_range(0..10)),
            Self::Num(n) => Value::Num(r.gen_range(0..*n) + 1),
            Self::Range(a, b) if a < b => Value::Num(r.gen_range(*a..*b)),
            Self::Range(b, a) => Value::Num(r.gen_range(*a..*b)),
            Self::Word(s)=> Value::Word(s.clone()),
            Self::List(v)=> {
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

impl PreValue {


    pub fn resolve(&self,c:&mut Context) ->anyhow::Result<Value> 
    {

    }



    pub fn flatten(&self, c: &mut Context) -> anyhow::Result<Vec<Value>> {
        match self{
            Self::Simple(v)=>(v.flatten()?),
            Self::Var(v)=>c.
    Var(String),
    Pop,
    List(Vec<PreValue>),
    Range(Box<PreValue>, Box<PreValue>),
    Roll(Box<PreValue>, Box<PreValue>),
        }
    }

    pub fn roll<R: Rng>(&self, r: &mut R) -> DiceValue {
        match self {
            Dice::D(10) => DiceValue::N(r.gen_range(0..10)),
            Dice::D(n) => DiceValue::N(r.gen_range(0..*n) + 1),
            Dice::R(a, b) if a < b => DiceValue::N(r.gen_range(*a..*b)),
            Dice::R(b, a) => DiceValue::N(r.gen_range(*a..*b)),
            Dice::F(v) => {
                let n = r.gen_range(0..v.len());
                v[n].clone()
            }
        }
    }
}
