#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    INT(u64),
    FLOAT(f64),
    NAME(String),
    PLUS,
    MINUS,
    MUL,
    DIV,
    MOD,
    POW,
    LPAREN,
    RPAREN,
    COMMA,
    EOF,
}
