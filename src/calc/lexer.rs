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

            if ch != '.' {
                let value = &self.expr[start..self.cursor];
                return Ok(Token::INT(value, start));
            }

            ch = self.advance();
            while ch.is_digit(10) {
                ch = self.advance();
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

// TODO - add documentation
// TODO - write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_valid_tokens() {
        let expr = "1+2-3*4/55%666^0";

        let mut lexer = Lexer::new(&expr);
        let result = lexer.tokenise();

        let expected = Ok(vec![
            Token::INT("1", 0),
            Token::PLUS(1),
            Token::INT("2", 2),
            Token::MINUS(3),
            Token::INT("3", 4),
            Token::MUL(5),
            Token::INT("4", 6),
            Token::DIV(7),
            Token::INT("55", 8),
            Token::MOD(10),
            Token::INT("666", 11),
            Token::POW(14),
            Token::INT("0", 15),
            Token::EOF(16),
        ]);

        assert_eq!(result, expected);
    }
}
