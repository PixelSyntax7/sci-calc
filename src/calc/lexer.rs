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

    #[allow(dead_code)]
    pub fn tokenise(&mut self) -> Result<Vec<Token>, String> {
        let mut found_eof_token = false;
        let mut tokens: Vec<Token> = Vec::new();

        while !found_eof_token {
            let token = self.next_token()?;
            match token {
                Token::EOF => found_eof_token = true,
                _ => {}
            }
            tokens.push(token);
        }

        return Ok(tokens);
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
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
                return Ok(Token::INT(value.parse::<u64>().unwrap()));
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
            return Ok(Token::FLOAT(value.parse::<f64>().unwrap()));
        }

        if ch.is_ascii_alphabetic() {
            ch = self.advance();
            while ch.is_ascii_alphabetic() {
                ch = self.advance();
            }
            let value = &self.expr[start..self.cursor];
            return Ok(Token::NAME(String::from(value).to_ascii_lowercase()));
        }

        if ch == '+' {
            self.advance();
            return Ok(Token::PLUS);
        }

        if ch == '-' {
            self.advance();
            return Ok(Token::MINUS);
        }

        if ch == '*' {
            self.advance();
            return Ok(Token::MUL);
        }

        if ch == '/' {
            self.advance();
            return Ok(Token::DIV);
        }

        if ch == '%' {
            self.advance();
            return Ok(Token::MOD);
        }

        if ch == '^' {
            self.advance();
            return Ok(Token::POW);
        }

        if ch == ',' {
            self.advance();
            return Ok(Token::COMMA);
        }

        if ch == '(' {
            self.advance();
            return Ok(Token::LPAREN);
        }

        if ch == ')' {
            self.advance();
            return Ok(Token::RPAREN);
        }

        if ch == '\0' {
            return Ok(Token::EOF);
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
#[allow(dead_code)]
pub fn tokenise<'a>(expr: &'a str) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(expr);
    lexer.tokenise()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_valid_operators() {
        assert_eq!(tokenise("+"), Ok(vec![Token::PLUS, Token::EOF]));
        assert_eq!(tokenise("-"), Ok(vec![Token::MINUS, Token::EOF]));
        assert_eq!(tokenise("*"), Ok(vec![Token::MUL, Token::EOF]));
        assert_eq!(tokenise("/"), Ok(vec![Token::DIV, Token::EOF]));
        assert_eq!(tokenise("%"), Ok(vec![Token::MOD, Token::EOF]));
        assert_eq!(tokenise("^"), Ok(vec![Token::POW, Token::EOF]));
    }

    #[test]
    fn tokenise_valid_delimiters() {
        assert_eq!(tokenise("("), Ok(vec![Token::LPAREN, Token::EOF]));
        assert_eq!(tokenise(")"), Ok(vec![Token::RPAREN, Token::EOF]));
        assert_eq!(tokenise(","), Ok(vec![Token::COMMA, Token::EOF]));
    }

    #[test]
    fn tokenise_valid_integer() {
        assert_eq!(tokenise("23"), Ok(vec![Token::INT(23), Token::EOF]));
        assert_eq!(
            tokenise("0023"),
            Ok(vec![Token::INT(23), Token::EOF])
        );
        assert_eq!(
            tokenise("0230"),
            Ok(vec![Token::INT(230), Token::EOF])
        );
    }

    #[test]
    fn tokenise_valid_float() {
        assert_eq!(
            tokenise("23.5"),
            Ok(vec![Token::FLOAT(23.5 as f64), Token::EOF])
        );
        assert_eq!(
            tokenise("23.500"),
            Ok(vec![Token::FLOAT(23.500 as f64), Token::EOF])
        );
        assert_eq!(
            tokenise("0.05"),
            Ok(vec![Token::FLOAT(0.05 as f64), Token::EOF])
        );
    }

    #[test]
    fn tokenise_valid_scientific_format() {
        assert_eq!(
            tokenise("5e10"),
            Ok(vec![Token::FLOAT(5e10 as f64), Token::EOF])
        );
        assert_eq!(
            tokenise("20.0E3"),
            Ok(vec![Token::FLOAT(20.0E3 as f64), Token::EOF])
        );
        assert_eq!(
            tokenise("5e+1"),
            Ok(vec![Token::FLOAT(5e+1 as f64), Token::EOF])
        );
        assert_eq!(
            tokenise("5e-10"),
            Ok(vec![Token::FLOAT(5e-10 as f64), Token::EOF])
        );
    }
}
