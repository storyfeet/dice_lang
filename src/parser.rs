use crate::expr::*;
use crate::tokenizer::{Token, TokenRes, TokenType, Tokenizer};
use err_tools::*;

pub fn parse_expr(s: &str) -> anyhow::Result<Expr> {
    let mut p = Parser::new(s);
    p.expr(0)?;
    Ok(p.target)
}

pub struct Parser<'a> {
    t: Tokenizer<'a>,
    peek: Option<Token<'a>>,
    target: Expr,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Parser {
            t: Tokenizer::new(s),
            peek: None,
            target: Expr::new(),
        }
    }
    pub fn emit(&mut self, op: Operation) {
        self.target.ops.push(op);
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

    pub fn expr(&mut self, prec: u32) -> anyhow::Result<()> {
        self.unary()?;
        while let Some(t) = self.peek_type() {
            let tp = t.precedence();
            if tp < prec {
                return Ok(());
            }
            self.binary(tp)?;
        }
        Ok(())
    }

    pub fn unary(&mut self) -> anyhow::Result<()> {
        let t = self.next_token()?.e_str("Expected Value found EOI")?;
        match t.tt {
            TokenType::Number(n) => self.emit(Operation::Num(n)),
            TokenType::Word(w) => {
                let ws = w.to_string();
                self.emit(Operation::Word(ws));
            }
            TokenType::P => self.emit(Operation::P),
            TokenType::F => self.emit(Operation::Fudge),
            TokenType::L => self.emit(Operation::L),
            TokenType::H => self.emit(Operation::H),
            TokenType::Dollar => {
                self.unary()?;
                self.emit(Operation::Var);
            }
            TokenType::D => {
                self.emit(Operation::Num(1));
                self.unary()?;
                self.emit(Operation::D);
            }
            t => return e_string(format!("Expected **Unary** operation found '{:?}'", t)),
        }
        Ok(())
    }

    pub fn binary(&mut self, _prec: u32) -> anyhow::Result<()> {
        match self.peek_type().e_str("Expected Token found EOI")? {
            TokenType::D => {
                self.peek = None;
                self.unary()?;
                self.emit(Operation::D);
            }
            TokenType::Add => {
                self.peek = None;
                self.unary()?;
                self.emit(Operation::Add);
            }
            TokenType::Sub => {
                self.peek = None;
                self.unary()?;
                self.emit(Operation::Sub);
            }
            t => return e_string(format!("Expected **Binary** operation found '{:?}'", t)),
        }
        Ok(())
    }

    pub fn list(&mut self) -> anyhow::Result<()> {
        self.consume_token(TokenType::BraceO)?;
        loop {
            match self.peek_type().e_str("Unclosed List")? {
                TokenType::BraceC => {
                    self.peek = None;
                    self.emit(Operation::List(0));
                    return Ok(());
                }
                _ => self.unary()?,
            }
        }
    }
}
