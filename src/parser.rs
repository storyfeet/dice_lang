use crate::expr::*;
use crate::tokenizer::{Token, TokenRes, TokenType, Tokenizer};
use err_tools::*;

macro_rules! op1 {
    ($x:ident,$s:ident) => {{
        $s.peek = None;
        let v = $s.value()?;
        Ok(Operation::$x(v))
    }};
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

    pub fn expr(&mut self) -> anyhow::Result<Expr> {
        let v = Box::new(self.value()?);
        let mut ops = Vec::new();
        while self.peek_type().is_some() {
            ops.push(self.operation()?);
        }
        Ok(Expr { v, ops })
    }

    pub fn value(&mut self) -> anyhow::Result<ExValue> {
        match self.peek_type().e_str("Expected Value found EOI")? {
            TokenType::D => Ok(ExValue::Num(1)),
            TokenType::ParenO => {
                self.peek = None;
                let res = self.expr()?;
                self.consume_token(TokenType::ParenC)
                    .e_str("No Close Bracket after Job")?;
                Ok(ExValue::Ex(res))
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
            TokenType::BraceO => self.list(),
            _ => e_str("Expected Value, got something else"),
        }
    }

    pub fn operation(&mut self) -> anyhow::Result<Operation> {
        match self.peek_type().e_str("Expected Token found EOI")? {
            TokenType::Add => op1!(Add, self),
            _ => {
                return e_str("Expected Operation, found something else");
            }
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
