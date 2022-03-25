use crate::expr::*;
use crate::tokenizer::{Token, TokenRes, TokenType, Tokenizer};
use err_tools::*;

macro_rules! bin_op {
    ($s:ident,$x:ident) => {{
        $s.peek = None;
        $s.unary()?;
        $s.emit(Operation::$x);
    }};
}

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
        let t = self.peek_type().e_str("Tried to consume EOI")?;
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
            TokenType::BraceO => {
                self.list()?;
            }
            t => return e_string(format!("Expected **Unary** operation found '{:?}'", t)),
        }
        Ok(())
    }

    pub fn binary(&mut self, prec: u32) -> anyhow::Result<()> {
        let t = self.peek_type().e_str("Expected Token found EOI")?;
        if t.precedence() < prec {
            return Ok(());
        }
        match t {
            TokenType::D => bin_op!(self, D),
            TokenType::Add => bin_op!(self, Add),
            TokenType::Sub => bin_op!(self, Sub),
            TokenType::Range => bin_op!(self, Range),
            t => return e_string(format!("Expected **Binary** operation found '{:?}'", t)),
        }
        Ok(())
    }

    pub fn list(&mut self) -> anyhow::Result<()> {
        self.peek = None; // TODO check if non null peek is BraceO
        let mut n = 0;
        loop {
            match self.peek_type().e_str("Unclosed List")? {
                TokenType::BraceC => {
                    self.peek = None;
                    self.emit(Operation::List(n));
                    return Ok(());
                }
                _ => {
                    self.unary()?;
                    n += 1;
                }
            }
        }
    }
}
