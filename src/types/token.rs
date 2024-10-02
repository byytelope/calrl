#[allow(dead_code)]
#[derive(Debug)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Divide,
    Multiply,
    Equals,
    ParanStart,
    ParanEnd,
    Eol,
}
