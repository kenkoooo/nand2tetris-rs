#[derive(Debug, Clone, Copy)]
pub enum VMCommand {
    Push(Segment, usize),
    Pop(Segment, usize),
    EQ,
    GT,
    LT,
    Add,
    Sub,
    And,
    Or,
    Neg,
    Not,
    Comment,
}

#[derive(Debug, Clone, Copy)]
pub enum Segment {
    Static,
    Local,
    Constant,
    Argument,
    This,
    That,
    Temp,
    Pointer,
}
