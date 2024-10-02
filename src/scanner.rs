use std::{iter::Peekable, str::CharIndices};

use crate::types::{errors::CalrlError, token::Token};

#[derive(Debug)]
pub struct Scanner<'a> {
    src: &'a str,
    pos: usize,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut chars = src.char_indices().peekable();

        Self {
            src,
            pos: chars.peek().map(|(x, _)| *x).unwrap_or_default(),
            chars,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_stream: Vec<Token> = vec![];

        while self.peek().is_some() {
            if let Ok(token) = self.advance() {
                token_stream.push(token);
            }
        }

        token_stream
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().map(|(_, ch)| ch)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|(_, ch)| *ch)
    }

    fn eat_while<F>(&mut self, f: F)
    where
        F: Fn(char) -> bool,
    {
        while self.peek().is_some() && f(self.peek().unwrap()) {
            self.next();
        }
    }

    fn slice(&mut self) -> &str {
        let end = self.cur_pos();
        &self.src[self.pos..end]
    }

    fn cur_pos(&mut self) -> usize {
        self.chars
            .peek()
            .map(|(idx, _)| *idx)
            .unwrap_or(self.src.len())
    }

    fn advance(&mut self) -> Result<Token, CalrlError> {
        self.eat_while(|ch| ch.is_whitespace());
        self.pos = self.cur_pos();

        let next_char = self.next().unwrap_or_default();

        Ok(match next_char {
            ch if ch.is_ascii_digit() => {
                self.eat_while(|ch| ch.is_ascii_digit());
                let num_str = self.slice();
                let num = num_str.parse::<i32>();

                match num {
                    Ok(n) => Token::Number(n),
                    Err(_) => return Err(CalrlError::ParseIntError),
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '(' => Token::ParanStart,
            ')' => Token::ParanEnd,
            '=' => Token::Equals,
            _ => Token::Eol,
        })
    }
}
