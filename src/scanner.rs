use crate::errors::CalrlError;
use std::{iter::Peekable, str::CharIndices};

const OPS: &str = "+-/*";

#[derive(Debug)]
pub enum Token {
    Number(String),
    Operator(String),
    Equals(String),
    ParanStart,
    ParanEnd,
    Eol,
}

#[derive(Debug)]
pub struct Scanner {
    pos: usize,
    src: &'static str,
    chars: Peekable<CharIndices<'static>>,
}

impl Scanner {
    pub fn new(src: &'static str) -> Self {
        let mut chars = src.char_indices().peekable();
        Self {
            pos: chars.peek().map(|(x, _)| *x).unwrap_or_default(),
            src,
            chars,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_stream: Vec<Token> = vec![];

        while self.peek().is_some() {
            let token = self.advance().unwrap();
            token_stream.push(token);
        }

        token_stream
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next().map(|(_, x)| x)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, x)| *x)
    }

    fn eat_while<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        while self.peek().is_some() && f(self.peek().unwrap()) {
            self.bump();
        }
    }

    fn slice(&mut self) -> &str {
        let end = self.cur_pos();
        &self.src[self.pos..end]
    }

    fn cur_pos(&mut self) -> usize {
        self.chars.peek().map(|(x, _)| *x).unwrap_or(self.src.len())
    }

    fn advance(&mut self) -> Result<Token, CalrlError> {
        // Remove whitespace
        self.eat_while(|x| matches!(x, ' ' | '\t' | '\n' | '\r'));
        self.pos = self.cur_pos();
        let next_char = if let Some(c) = self.bump() {
            c
        } else {
            return Ok(Token::Eol);
        };

        match next_char {
            x if x.is_ascii_digit() => {
                self.eat_while(|x| x.is_ascii_digit());
                let num = self.slice();

                Ok(Token::Number(num.into()))
            }
            x if OPS.contains(x) => Ok(Token::Operator(x.into())),
            _ => Err(CalrlError::UnexpectedChar(next_char, self.cur_pos())),
        }
    }
}
