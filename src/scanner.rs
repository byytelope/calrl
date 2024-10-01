use std::{iter::Peekable, str::CharIndices};

const OPS: &str = "+-/*";

#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Number(String),
    Operator(String),
    Equals,
    ParanStart,
    ParanEnd,
}

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

    pub fn eval(&mut self) {
        let tokens = self.lex();
        println!("{:#?}", tokens);
    }

    fn lex(&mut self) -> Vec<Token> {
        let mut token_stream: Vec<Token> = vec![];

        while self.peek().is_some() {
            if let Some(token) = self.advance() {
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

    fn advance(&mut self) -> Option<Token> {
        self.eat_while(|ch| ch.is_whitespace());
        self.pos = self.cur_pos();

        let next_char = self.next().unwrap_or_default();

        Some(match next_char {
            ch if ch.is_ascii_digit() => {
                self.eat_while(|ch| ch.is_ascii_digit());
                let num = self.slice();

                Token::Number(num.into())
            }
            ch if OPS.contains(ch) => Token::Operator(ch.into()),
            '(' => Token::ParanStart,
            ')' => Token::ParanEnd,
            '=' => Token::Equals,
            _ => return None,
        })
    }
}
