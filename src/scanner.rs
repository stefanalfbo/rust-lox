use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<(usize, String)>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            b'!' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            b'=' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            b'<' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            b'>' => {
                let token_type = if self.match_char(b'=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            b'/' => {
                if self.match_char(b'/') {
                    // A comment goes until the end of the line.
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            b' ' | b'\r' | b'\t' => {
                // Ignore whitespace.
            }
            b'\n' => {
                self.line += 1;
            }
            b'"' => self.string(),
            ch if ch.is_ascii_digit() => self.number(),
            ch if ch.is_ascii_alphabetic() || ch == b'_' => self.identifier(),
            _ => self
                .errors
                .push((self.line, format!("Unexpected character: {}", c as char))),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), None, self.line));
    }

    fn match_char(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        self.source.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0';
        }
        self.source.as_bytes()[self.current + 1]
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push((self.line, "Unterminated string.".to_string()));
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        // self.add_token(TokenType::String);
        self.tokens.push(Token::new(
            TokenType::String,
            value.to_string(),
            Some(value.to_string()),
            self.line,
        ));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        // self.add_token(TokenType::Number);
        self.tokens.push(Token::new(
            TokenType::Number,
            value.to_string(),
            Some(value.to_string()),
            self.line,
        ));
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == b'_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        self.tokens
            .push(Token::new(token_type, text.to_string(), None, self.line));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scan(source: &str) -> Scanner<'_> {
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens();
        scanner
    }

    fn assert_token(
        token: &Token,
        token_type: TokenType,
        lexeme: &str,
        literal: Option<&str>,
        line: usize,
    ) {
        assert!(token_type_eq(&token.token_type, &token_type));
        assert_eq!(token.lexeme, lexeme);
        assert_eq!(token.literal.as_deref(), literal);
        assert_eq!(token.line, line);
    }

    fn token_type_eq(left: &TokenType, right: &TokenType) -> bool {
        std::mem::discriminant(left) == std::mem::discriminant(right)
    }

    #[test]
    fn scan_single_char_tokens() {
        let scanner = scan("(){}.,-+;*/");
        let tokens = scanner.tokens;

        let expected = [
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Dot,
            TokenType::Comma,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Star,
            TokenType::Slash,
            TokenType::Eof,
        ];

        assert_eq!(tokens.len(), expected.len());
        for (token, token_type) in tokens.iter().zip(expected.iter()) {
            assert!(token_type_eq(&token.token_type, token_type));
        }
    }

    #[test]
    fn scan_two_char_tokens() {
        let scanner = scan("! != = == < <= > >=");
        let tokens = scanner.tokens;

        assert_token(&tokens[0], TokenType::Bang, "!", None, 1);
        assert_token(&tokens[1], TokenType::BangEqual, "!=", None, 1);
        assert_token(&tokens[2], TokenType::Equal, "=", None, 1);
        assert_token(&tokens[3], TokenType::EqualEqual, "==", None, 1);
        assert_token(&tokens[4], TokenType::Less, "<", None, 1);
        assert_token(&tokens[5], TokenType::LessEqual, "<=", None, 1);
        assert_token(&tokens[6], TokenType::Greater, ">", None, 1);
        assert_token(&tokens[7], TokenType::GreaterEqual, ">=", None, 1);
        assert!(token_type_eq(&tokens[8].token_type, &TokenType::Eof));
    }

    #[test]
    fn scan_numbers_and_strings() {
        let scanner = scan("123 45.67 \"hi\"");
        let tokens = scanner.tokens;

        assert_token(&tokens[0], TokenType::Number, "123", Some("123"), 1);
        assert_token(&tokens[1], TokenType::Number, "45.67", Some("45.67"), 1);
        assert_token(&tokens[2], TokenType::String, "hi", Some("hi"), 1);
        assert!(token_type_eq(&tokens[3].token_type, &TokenType::Eof));
    }

    #[test]
    fn scan_identifiers_and_keywords() {
        let scanner = scan("and class foo bar_1 var while");
        let tokens = scanner.tokens;

        assert_token(&tokens[0], TokenType::And, "and", None, 1);
        assert_token(&tokens[1], TokenType::Class, "class", None, 1);
        assert_token(&tokens[2], TokenType::Identifier, "foo", None, 1);
        assert_token(&tokens[3], TokenType::Identifier, "bar_1", None, 1);
        assert_token(&tokens[4], TokenType::Var, "var", None, 1);
        assert_token(&tokens[5], TokenType::While, "while", None, 1);
        assert!(token_type_eq(&tokens[6].token_type, &TokenType::Eof));
    }

    #[test]
    fn scan_comments_and_line_numbers() {
        let scanner = scan("var a = 1; // comment\nprint a;");
        let tokens = scanner.tokens;

        assert_token(&tokens[0], TokenType::Var, "var", None, 1);
        assert_token(&tokens[1], TokenType::Identifier, "a", None, 1);
        assert_token(&tokens[2], TokenType::Equal, "=", None, 1);
        assert_token(&tokens[3], TokenType::Number, "1", Some("1"), 1);
        assert_token(&tokens[4], TokenType::Semicolon, ";", None, 1);
        assert_token(&tokens[5], TokenType::Print, "print", None, 2);
        assert_token(&tokens[6], TokenType::Identifier, "a", None, 2);
        assert_token(&tokens[7], TokenType::Semicolon, ";", None, 2);
        assert!(token_type_eq(&tokens[8].token_type, &TokenType::Eof));
    }

    #[test]
    fn scan_unterminated_string_records_error() {
        let scanner = scan("\"unterminated");

        assert_eq!(scanner.tokens.len(), 1);
        assert!(token_type_eq(
            &scanner.tokens[0].token_type,
            &TokenType::Eof
        ));
        assert_eq!(scanner.errors.len(), 1);
        assert_eq!(scanner.errors[0].0, 1);
        assert_eq!(scanner.errors[0].1, "Unterminated string.");
    }
}
