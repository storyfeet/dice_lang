use rand::*;

#[derive(Clone, Debug)]
pub enum DiceValue {
    S(String),
    N(i32),
}

impl DiceValue {
    pub fn as_int(&self) -> anyhow::Result<i32> {
        match self {
            Self::S(_) => err_tools::e_str("Cannot use String as Number"),
            Self::N(n) => Ok(*n),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DiceResult(pub Vec<DiceValue>);

impl From<i32> for DiceResult {
    fn from(n: i32) -> DiceResult {
        DiceResult(vec![DiceValue::N(n)])
    }
}

pub struct NDice {
    pub n: i32,
    pub d: Dice,
}

impl NDice {
    pub fn roll(&self) -> DiceResult {
        let mut v = Vec::new();
        let mut tr = rand::thread_rng();
        for _ in 0..self.n {
            v.push(self.d.roll(&mut tr))
        }
        DiceResult(v)
    }
}

pub enum Dice {
    D(i32),
    R(i32, i32),
    F(Vec<DiceValue>),
}

impl Dice {
    pub fn n(self, n: i32) -> NDice {
        NDice { n, d: self }
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
