#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Operator(Operation),
    Equals,
    ParanStart,
    ParanEnd,
    Eol,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}
