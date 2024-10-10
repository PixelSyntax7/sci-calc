#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    INT(&'a str, usize),
    FLOAT(&'a str, usize),
    NAME(&'a str, usize),
    PLUS(usize),
    MINUS(usize),
    MUL(usize),
    DIV(usize),
    MOD(usize),
    POW(usize),
    LPAREN(usize),
    RPAREN(usize),
    COMMA(usize),
    EOF(usize),
}
