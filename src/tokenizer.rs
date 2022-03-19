use err_tools::*;
use std::str::CharIndices;

pub enum TokenType {
    D,
    Number(u32),
    Word,
}

pub struct Token<'a> {
    s: &'a str,
    tt: TokenType,
    start: usize,
    end: usize,
    line: usize,
}

pub struct Tokenizer<'a> {
    s: &'a str,
    chars: CharIndices<'a>,
    index: usize,
    line: usize,
    peek: Option<(usize, char)>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s: s,
            chars: s.char_indices(),
            index: 0,
            line: 0,
            peek: None,
        }
    }

    pub fn next_char(&mut self) -> Option<(usize, char)> {
        match self.peek.take() {
            Some(c) => Some(c),
            None => self.chars.next(),
        }
    }

    pub fn peek_char(&mut self) -> Option<(usize, char)> {
        let (ni, nc) = self.next_char()?;
        self.peek = Some((ni, nc));
        Some((ni, nc))
    }

    pub fn peek_index(&mut self) -> usize {
        match self.peek_char() {
            None => self.s.len(),
            Some((i, _)) => i,
        }
    }

    pub fn make_token(&mut self, tt: TokenType) -> Token<'a> {
        let start = self.index;
        let end = self.peek_index();
        Token {
            start,
            end,
            s: &self.s[start..end],
            tt,
            line: self.line,
        }
    }

    pub fn number(&mut self) -> Option<anyhow::Result<Token<'a>>> {
        let mut res = 0;
        let found = false;
        loop {
            match self.next_char() {
                None => {
                    if found {
                        return Some(Ok(self.make_token(TokenType::Number(res))));
                    } else {
                        return Some(e_str("No Number Digits found in number method"));
                    }
                }
                Some(n) if n >= '0' && n <= '9'{
                    res
                }
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = anyhow::Result<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index = self.peek_index();
        match self.next_char() {
            None => None,
            Some((i, c)) if c >= '0' && c <= '9' => {
                self.peek = Some((i, c));
                self.number()
            }
        }
    }
}
