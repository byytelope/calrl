#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Token {
    Number(f32),
    Operator(Operation),
    ParanStart,
    ParanEnd,
}

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}
