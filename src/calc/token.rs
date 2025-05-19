#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    INT(&'a str),
    FLOAT(&'a str),
    NAME(&'a str),
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
