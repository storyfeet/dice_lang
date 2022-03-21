use err_tools::*;
use std::str::CharIndices;

pub type TokenRes<'a> = Option<anyhow::Result<Token<'a>>>;

#[derive(Debug, PartialEq)]
pub enum TokenType<'a> {
    D,
    Number(i32),
    Word(&'a str),
    OpenB,
    CloseB,
    Sub,
    Add,
}

pub struct Token<'a> {
    s: &'a str,
    tt: TokenType<'a>,
    start: usize,
    end: usize,
}

pub struct Tokenizer<'a> {
    s: &'a str,
    chars: CharIndices<'a>,
    start: usize,
    peek: Option<(usize, char)>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s: s,
            chars: s.char_indices(),
            start: 0,
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

    pub fn make_token_wrap(&mut self, tt: TokenType<'a>) -> TokenRes<'a> {
        Some(Ok(self.make_token(tt)))
    }

    pub fn make_token(&mut self, tt: TokenType<'a>) -> Token<'a> {
        let start = self.start;
        let end = self.peek_index();
        self.start = end;
        Token {
            start,
            end,
            s: &self.s[start..end],
            tt,
        }
    }

    pub fn number(&mut self) -> Option<anyhow::Result<Token<'a>>> {
        let mut res: i32 = 0;
        let mut found = false;
        loop {
            match self.peek_char() {
                Some((_, n)) if n >= '0' && n <= '9' => {
                    res = res * 10 + (n as i32 - '0' as i32);
                    found = true;
                    self.peek = None;
                }
                _ => {
                    if found {
                        return Some(Ok(self.make_token(TokenType::Number(res))));
                    } else {
                        return Some(e_str("No Number Digits found in number method"));
                    }
                }
            }
        }
    }

    pub fn qoth(&mut self) -> Option<anyhow::Result<Token<'a>>> {
        self.peek = None;
        let start = self.peek_index();
        loop {
            match self.next_char() {
                Some((end, '\"')) => {
                    return self.make_token_wrap(TokenType::Word(&self.s[start..end]));
                }
                Some(_) => {}
                None => return Some(e_str("EOI inside quotes")),
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = anyhow::Result<Token<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.peek_index();
        match self.peek_char() {
            None => None,
            Some((i, c)) if c >= '0' && c <= '9' => {
                self.peek = Some((i, c));
                self.number()
            }
            Some((_, '\"')) => self.qoth(),
            Some((_, '(')) => return self.make_token_wrap(TokenType::OpenB),
            Some((_, ')')) => return self.make_token_wrap(TokenType::CloseB),
            Some((_, '+')) => {
                self.peek = None;
                return self.make_token_wrap(TokenType::Add);
            }
            Some(_) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod token_test {
    use super::*;

    #[test]
    pub fn test_tokenizer() {
        let s = r#""hello"29+"food""#;
        let mut tk = Tokenizer::new(s);
        let mut t = tk.next().unwrap().unwrap();
        assert_eq!(t.start, 0);
        assert_eq!(t.end, 7);
        assert_eq!(t.s, r#""hello""#);
        assert_eq!(t.tt, TokenType::Word("hello"));
        t = tk.next().unwrap().unwrap();
        assert_eq!(t.tt, TokenType::Number(29));
        t = tk.next().unwrap().unwrap();
        assert_eq!(t.tt, TokenType::Add, "Gobble");
        t = tk.next().unwrap().unwrap();
        assert_eq!(t.tt, TokenType::Word("food"));
    }
}
