use crate::types::{
    errors::CalrlError,
    token::{Operation, Token},
};

#[derive(Debug)]
pub struct Engine {
    tokens: Vec<Token>,
}

impl Engine {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn eval(&self) -> Result<f32, CalrlError> {
        let mut stack: Vec<f32> = vec![];
        let shunted = self.shunt();

        for token in shunted {
            match token {
                Token::Number(num) => stack.push(num),
                Token::Operator(op) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    match op {
                        Operation::Add => stack.push(left + right),
                        Operation::Subtract => stack.push(left - right),
                        Operation::Multiply => stack.push(left * right),
                        Operation::Divide => stack.push(left / right),
                    }
                }
                _ => {}
            }
        }

        Ok(stack.pop().unwrap())
    }

    fn shunt(&self) -> Vec<Token> {
        let mut output: Vec<Token> = vec![];
        let mut stack: Vec<Token> = vec![];

        for token in &self.tokens {
            match token {
                Token::Number(_) => output.push(*token),
                Token::Operator(op) => match op {
                    Operation::Multiply | Operation::Divide => stack.push(*token),
                    Operation::Add | Operation::Subtract => {
                        if matches!(
                            stack.last(),
                            Some(Token::Operator(Operation::Divide | Operation::Multiply))
                        ) {
                            output.push(stack.pop().unwrap());
                        }

                        stack.push(*token);
                    }
                },
                Token::ParanStart => stack.push(*token),
                Token::ParanEnd => {
                    while !matches!(stack.last(), Some(Token::ParanStart)) {
                        output.push(stack.pop().unwrap())
                    }
                    stack.pop().unwrap();
                }
            }
        }

        while let Some(token) = stack.pop() {
            output.push(token);
        }

        output
    }
}
