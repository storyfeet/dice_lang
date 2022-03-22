use crate::dice::Dice;
use crate::instruction::Job;
use crate::tokenizer::{Token, TokenRes, TokenType, Tokenizer};
use err_tools::*;

pub struct Parser<'a> {
    t: Tokenizer<'a>,
    peek: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Parser {
            t: Tokenizer::new(s),
            peek: None,
        }
    }

    pub fn next_token(&mut self) -> TokenRes<'a> {
        match self.peek.take() {
            Some(t) => Ok(Some(t)),
            None => self.t.next(),
        }
    }

    pub fn peek_token(&mut self) -> TokenRes<'a> {
        match self.next_token()? {
            Some(t) => Ok(Some(t.clone())),
            None => Ok(None),
        }
    }

    pub fn peek_type(&mut self) -> Option<TokenType> {
        match self.peek_token() {
            Err(_) => None,
            Ok(Some(t)) => Some(t.tt.clone()),
            Ok(None) => None,
        }
    }

    pub fn consume_token(&mut self, tt: TokenType<'a>) -> anyhow::Result<()> {
        let t = self.peek_type().e_str("Tried to consume Non Token")?;
        if t == tt {
            self.peek = None;
            return Ok(());
        }
        return e_str("Consume token, required token did not match");
    }

    pub fn job(&mut self) -> anyhow::Result<Job> {
        let mut js = self.value()?;
        loop {
            // Try instructions and see what can be added
            match self.peek_type() {
                Some(TokenType::ParenC) => return Ok(js),
                None => return Ok(js),
                Some(TokenType::Add) => {
                    self.peek = None,

                }
                _ => unimplemented!(),
            }
        }
    }

    pub fn value(&mut self) -> anyhow::Result<Job> {
        match self.peek_type() {
            Some(TokenType::ParenO) => {
                self.peek = None;
                let res = self.job()?;
                self.consume_token(TokenType::ParenO)
                    .e_str("No Close Bracket after Job")?;
                Ok(res)
            }
            Some(TokenType::Number(n)) => {
                self.peek = None;
                if let Some(TokenType::D) = self.peek_type() {
                    self.peek = None;
                    let ndice = self.faces()?.n(n);
                    Ok(Job::dice(ndice))
                } else {
                    Ok(Job::num(n))
                }
            }
        }
    }

    pub fn faces(&mut self) -> anyhow::Result<Dice> {
        match self.peek_type().e_str("Dice need faces")?{
            TokenType::Number(n) => {
                self.peek = None;
                Ok(Dice::D(n))
            }
            TokenType::BraceO => {
                self.face_list()
            }
        }
    }

    pub fn v_list
}
