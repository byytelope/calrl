use crate::types::{errors::CalrlError, token::Token};

#[derive(Debug)]
pub struct Engine {
    tokens: Vec<Token>,
}

impl Engine {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn eval(&self) -> Result<i32, CalrlError> {
        self.group();

        Ok(0)
    }

    fn group(&self) -> Vec<Vec<Token>> {
        let groups: Vec<Vec<Token>> = vec![];
        // for (idx, token) in self.tokens.iter().enumerate() {
        //     match token {
        //         Token::Number(_) => todo!(),
        //         Token::Plus => todo!(),
        //         Token::Minus => todo!(),
        //         Token::Divide => todo!(),
        //         Token::Multiply => todo!(),
        //         Token::Equals => todo!(),
        //         Token::ParanStart => todo!(),
        //         Token::ParanEnd => todo!(),
        //         Token::Eol => todo!(),
        //     }
        // }
        println!("{:#?}", self.tokens);

        groups
    }
}
