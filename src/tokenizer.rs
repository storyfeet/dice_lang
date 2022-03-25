use err_tools::*;
use std::str::CharIndices;

pub type TokenRes<'a> = anyhow::Result<Option<Token<'a>>>;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    D,
    L,
    H,
    P,
    F,
    Number(i32),
    Word(&'a str),
    ParenO,
    ParenC,
    BraceO,
    BraceC,
    Dollar,
    Sub,
    Add,
    Push,
    Pop,
    Range,
    Colon,
}

impl<'a> TokenType<'a> {
    pub fn from_word(s: &'a str) -> Self {
        match s {
            "D" | "d" => TokenType::D,
            "push" => TokenType::Push,
            "pop" => TokenType::Pop,
            "P" => TokenType::P,
            "H" => TokenType::H,
            "L" => TokenType::L,
            "F" => TokenType::F,
            s => TokenType::Word(s),
        }
    }

    pub fn precedence(&self) -> u32 {
        match self {
            Self::L => 1,
            Self::H => 1,
            Self::P => 1,
            Self::F => 1,
            Self::Number(_) => 1,
            Self::Word(_) => 1,
            Self::Pop => 2,
            Self::Push => 3,
            Self::Add => 4,
            Self::Sub => 5,
            Self::Colon => 7,
            Self::D => 9,
            Self::Range => 10,
            Self::ParenO => 11,
            Self::ParenC => 11,
            Self::BraceO => 11,
            Self::BraceC => 11,
            Self::Dollar => 12,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub s: &'a str,
    pub tt: TokenType<'a>,
    pub start: usize,
    pub end: usize,
}

pub fn print_tokens(s: &str) {
    let mut t = Tokenizer::new(s);
    while let Ok(Some(t)) = t.next() {
        println!("T:{:?}", t);
    }
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

    pub fn make_token_wrap(&mut self, tt: TokenType<'a>, unpeek: bool) -> TokenRes<'a> {
        if unpeek {
            self.peek = None
        }
        Ok(Some(self.make_token(tt)))
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

    pub fn number(&mut self) -> TokenRes<'a> {
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
                        return self.make_token_wrap(TokenType::Number(res), false);
                    } else {
                        return e_str("No Number Digits found in number method");
                    }
                }
            }
        }
    }

    pub fn unqoth(&mut self) -> TokenRes<'a> {
        let start = self.peek_index();
        loop {
            match self.peek_char() {
                Some((_, c)) if c.is_alphabetic() => self.peek = None,
                Some((i, _)) => {
                    return self.make_token_wrap(TokenType::from_word(&self.s[start..i]), false)
                }
                None => return self.make_token_wrap(TokenType::from_word(&self.s[start..]), false),
            }
        }
    }

    pub fn qoth(&mut self) -> TokenRes<'a> {
        self.peek = None;
        let start = self.peek_index();
        loop {
            match self.next_char() {
                Some((end, '\"')) => {
                    return self.make_token_wrap(TokenType::Word(&self.s[start..end]), false);
                }
                Some(_) => {}
                None => return e_str("EOI inside quotes"),
            }
        }
    }

    pub fn white_space(&mut self) {
        loop {
            match self.peek_char() {
                Some((_, c)) if c.is_whitespace() => {
                    self.peek = None;
                }
                _ => return,
            }
        }
    }
    pub fn next(&mut self) -> TokenRes<'a> {
        self.white_space();
        self.start = self.peek_index();
        match self.peek_char() {
            None => Ok(None),
            Some((i, c)) if c >= '0' && c <= '9' => {
                self.peek = Some((i, c));
                self.number()
            }
            Some((_, '\"')) => self.qoth(),
            Some((_, '(')) => self.make_token_wrap(TokenType::ParenO, true),
            Some((_, ')')) => self.make_token_wrap(TokenType::ParenC, true),
            Some((_, '[')) => self.make_token_wrap(TokenType::BraceO, true),
            Some((_, ']')) => self.make_token_wrap(TokenType::BraceC, true),
            Some((_, '+')) => self.make_token_wrap(TokenType::Add, true),
            Some((_, '-')) => self.make_token_wrap(TokenType::Sub, true),
            Some((_, '$')) => self.make_token_wrap(TokenType::Dollar, true),
            Some((_, ':')) => self.make_token_wrap(TokenType::Colon, true),
            Some((_, '.')) => {
                //todo, allow single dot variable path
                self.peek = None;
                match self.next_char() {
                    Some((_, '.')) => self.make_token_wrap(TokenType::Range, false),
                    _ => e_str("Expected second Dot"),
                }
            }
            Some((_, c)) if c.is_alphabetic() => self.unqoth(),

            Some(_) => e_str("Unexpected Character"),
        }
    }
}

#[cfg(test)]
mod token_test {
    use super::*;

    #[test]
    pub fn test_tokenizer() {
        let s = r#""hello" 29+food3"#;
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
        t = tk.next().unwrap().unwrap();
        assert_eq!(t.tt, TokenType::Number(3));
        assert!(tk.next().is_none());
    }
}
