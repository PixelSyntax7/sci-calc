use crate::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub expr: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(expr: &'a str) -> Lexer<'a> {
        Lexer { expr, cursor: 0 }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut found_eof_token = false;
        let mut tokens: Vec<Token<'a>> = Vec::new();

        while !found_eof_token {
            let token = self.next_token()?;
            match token {
                Token::EOF(_) => found_eof_token = true,
                _ => {}
            }
            tokens.push(token);
        }

        return Ok(tokens);
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, String> {
        let mut ch = self.curr_char();

        while ch == ' ' {
            ch = self.advance();
        }

        let start = self.cursor;

        if ch.is_digit(10) {
            ch = self.advance();
            while ch.is_digit(10) {
                ch = self.advance();
            }

            if ch != '.' && ch != 'e' && ch != 'E' {
                let value = &self.expr[start..self.cursor];
                return Ok(Token::INT(value, start));
            }

            if ch == '.' {
                ch = self.advance();
                while ch.is_digit(10) {
                    ch = self.advance();
                }
            }

            if ch == 'e' || ch == 'E' {
                ch = self.advance();
                if ch == '+' || ch == '-' {
                    ch = self.advance();
                }
                while ch.is_digit(10) {
                    ch = self.advance();
                }
            }

            let value = &self.expr[start..self.cursor];
            return Ok(Token::FLOAT(value, start));
        }

        if ch.is_ascii_alphabetic() {
            ch = self.advance();
            while ch.is_ascii_alphabetic() {
                ch = self.advance();
            }
            let value = &self.expr[start..self.cursor];
            return Ok(Token::NAME(value, start));
        }

        if ch == '+' {
            self.advance();
            return Ok(Token::PLUS(start));
        }

        if ch == '-' {
            self.advance();
            return Ok(Token::MINUS(start));
        }

        if ch == '*' {
            self.advance();
            return Ok(Token::MUL(start));
        }

        if ch == '/' {
            self.advance();
            return Ok(Token::DIV(start));
        }

        if ch == '%' {
            self.advance();
            return Ok(Token::MOD(start));
        }

        if ch == '^' {
            self.advance();
            return Ok(Token::POW(start));
        }

        if ch == ',' {
            self.advance();
            return Ok(Token::COMMA(start));
        }

        if ch == '(' {
            self.advance();
            return Ok(Token::LPAREN(start));
        }

        if ch == ')' {
            self.advance();
            return Ok(Token::RPAREN(start));
        }

        if ch == '\0' {
            return Ok(Token::EOF(start));
        }

        let msg = format!("Unknown character '{}'", ch);
        return Err(msg);
    }

    /// character under the cursor
    fn curr_char(&mut self) -> char {
        self.expr.chars().nth(self.cursor).unwrap_or('\0')
    }

    /// move to next character if can
    /// and return the character under the cursor
    fn advance(&mut self) -> char {
        if self.cursor >= self.expr.len() {
            return '\0';
        }
        self.cursor += 1;
        self.curr_char()
    }
}

/// Provides the tokens from expression
pub fn tokenise<'a>(expr: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut lexer = Lexer::new(expr);
    lexer.tokenise()
}

// TODO - docs
// TODO - tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_valid_operators() {
        assert_eq!(tokenise("+"), Ok(vec![Token::PLUS(0), Token::EOF(1)]));
        assert_eq!(tokenise("-"), Ok(vec![Token::MINUS(0), Token::EOF(1)]));
        assert_eq!(tokenise("*"), Ok(vec![Token::MUL(0), Token::EOF(1)]));
        assert_eq!(tokenise("/"), Ok(vec![Token::DIV(0), Token::EOF(1)]));
        assert_eq!(tokenise("%"), Ok(vec![Token::MOD(0), Token::EOF(1)]));
        assert_eq!(tokenise("^"), Ok(vec![Token::POW(0), Token::EOF(1)]));
    }

    #[test]
    fn tokenise_valid_delimiters() {
        assert_eq!(tokenise("("), Ok(vec![Token::LPAREN(0), Token::EOF(1)]));
        assert_eq!(tokenise(")"), Ok(vec![Token::RPAREN(0), Token::EOF(1)]));
        assert_eq!(tokenise(","), Ok(vec![Token::COMMA(0), Token::EOF(1)]));
    }

    #[test]
    fn tokenise_valid_integer() {
        assert_eq!(tokenise("23"), Ok(vec![Token::INT("23", 0), Token::EOF(2)]));
        assert_eq!(
            tokenise("0023"),
            Ok(vec![Token::INT("0023", 0), Token::EOF(4)])
        );
        assert_eq!(
            tokenise("0230"),
            Ok(vec![Token::INT("0230", 0), Token::EOF(4)])
        );
    }

    #[test]
    fn tokenise_valid_float() {
        assert_eq!(
            tokenise("23.5"),
            Ok(vec![Token::FLOAT("23.5", 0), Token::EOF(4)])
        );
        assert_eq!(
            tokenise("23.500"),
            Ok(vec![Token::FLOAT("23.500", 0), Token::EOF(6)])
        );
        assert_eq!(
            tokenise("0.05"),
            Ok(vec![Token::FLOAT("0.05", 0), Token::EOF(4)])
        );
    }

    #[test]
    fn tokenise_valid_scientific_format() {
        assert_eq!(
            tokenise("5e10"),
            Ok(vec![Token::FLOAT("5e10", 0), Token::EOF(4)])
        );
        assert_eq!(
            tokenise("20.0E3"),
            Ok(vec![Token::FLOAT("20.0E3", 0), Token::EOF(6)])
        );
        assert_eq!(
            tokenise("5e+1"),
            Ok(vec![Token::FLOAT("5e+1", 0), Token::EOF(4)])
        );
        assert_eq!(
            tokenise("5e-10"),
            Ok(vec![Token::FLOAT("5e-10", 0), Token::EOF(5)])
        );
    }
}
