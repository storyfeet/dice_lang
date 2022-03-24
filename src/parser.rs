use crate::expr::*;
use crate::tokenizer::{Token, TokenRes, TokenType, Tokenizer};
use err_tools::*;



pub fn parse_expr(s: &str) -> anyhow::Result<Expr> {
    let mut p = Parser::new(s);
    p.expr(0)
}

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
            Some(t) => {
                self.peek = Some(t.clone());
                Ok(Some(t))
            }
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

    pub fn expr(&mut self, prec: u32) -> anyhow::Result<Expr> {
        let v = self.value()?;
        match self.peek_type(){
            None => {
                return Ok(Expr::new(v,Operation::Value))
            }
            Some(t) if t.precedence < prec =>{
                return Ok(Expr::new(v,Operation::Value))
            }
        }
        
        while let Some(t) = self.peek_type() {
            if t.precedence()
            if t == TokenType::ParenC {
                break;
            }
            ops.push(self.operation()?);
        }
        Ok(Expr { v, ops })
    }

    pub fn value(&mut self) -> anyhow::Result<ExValue> {
        match self.peek_type().e_str("Expected Value found EOI")? {
            TokenType::D => {
                self.peek = None;
                let res = self.expr(11)?;
                Ok(ExValue::D(Box::new(res)))
            }
            TokenType::Sub => {
                self.peek = None;
                let res = self.expr(5)?;
                Ok(ExValue::Neg(Box::new(res)))
            }
            TokenType::Sub => Ok(ExValue::Num(0)),
            TokenType::ParenO => {
                self.peek = None;
                let res = self.expr(0)?;
                self.consume_token(TokenType::ParenC)
                    .e_str("No Close Bracket after Job")?;
                Ok(ExValue::Ex(Box::new(res)))
            }
            TokenType::Number(n) => {
                self.peek = None;
                Ok(ExValue::Num(n))
            }
            TokenType::Word(w) => {
                let w = w.to_string();
                self.peek = None;
                Ok(ExValue::Word(w))
            }
            TokenType::L => val0!(L, self),
            TokenType::H => val0!(H, self),
            TokenType::P => val0!(P, self),
            TokenType::F => val0!(Fudge, self),
            TokenType::BraceO => self.list(),
            _ => e_str("Expected Value, got something else"),
        }
    }

    pub fn operation(&mut self, precedence: u32) -> anyhow::Result<Operation> {
        match self.peek_type().e_str("Expected Token found EOI")? {
            TokenType::Add => op1!(Add, self),
            TokenType::Sub => op1!(Sub, self),
            TokenType::D => op1!(D, self),
            TokenType::Colon => op1!(Replace, self),
            TokenType::Range => op1!(Range, self),
            t => e_string(format!("Expected Operation, found {:?}", t)),
        }
    }

    pub fn list(&mut self) -> anyhow::Result<ExValue> {
        self.consume_token(TokenType::BraceO)?;
        let mut res = Vec::new();
        loop {
            match self.peek_type().e_str("Unclosed List")? {
                TokenType::BraceC => {
                    self.peek = None;
                    return Ok(ExValue::List(res));
                }
                _ => res.push(self.value()?),
            }
        }
    }
}
