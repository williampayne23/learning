use rstest::rstest;
use crate::token;


pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l
    }
}

impl Lexer {
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        if let Some(tok) = self.try_single_char_token() {
            return tok;
        }

        if let Some(tok) = self.try_ident_or_keyword() {
            return tok;
        }

        if let Some(tok) = self.try_number() {
            return tok;
        }

        if let Some(tok) = self.try_eof() {
            return tok;
        }

        return token::Token::new_illegal(&self.ch.to_string());
    }

    fn try_single_char_token(&mut self) -> Option<token::Token> {
        let token_type = token::lookup_single_char_token(self.ch);
        if let Some(token_type) = token_type {
            let tok = token::Token::new_symbol(token_type);
            self.read_char();
            return Some(tok);
        }
        return None;
    } 

    fn try_ident_or_keyword(&mut self) -> Option<token::Token> {
        if is_identifier_char(self.ch) {
            let start = self.position;
            let mut end = start;
            while is_identifier_char(self.ch) {
                println!("ch: {}", self.ch);
                end = self.position;
                self.read_char();
            }
            let literal = self.input[start..=end].to_string();
            if let Some(token_type) = token::lookup_keyword(&literal) {
                return Some(token::Token::new_symbol(token_type));
            }
            return Some(token::Token::new_ident(&literal));
        }
        return None;
    }

    fn try_number(&mut self) -> Option<token::Token> {
        if self.ch.is_numeric() {
            let start = self.position;
            let mut end = start;
            while self.ch.is_numeric() {
                end = self.position;
                self.read_char();
            }
            let literal = self.input[start..=end].to_string();
            return Some(token::Token::new_number(&literal));
        }
        return None;
    }

    fn try_eof(&self) -> Option<token::Token> {
        if self.ch == '\0' {
            return Some(token::Token::new_symbol(token::TokenType::EOF));
        }
        return None;
    }
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}


#[rstest]
#[case(
    "-/=*".to_string(),
    vec![
        token::Token::new_symbol(token::TokenType::Minus),
        token::Token::new_symbol(token::TokenType::Slash),
        token::Token::new_symbol(token::TokenType::Assign),
        token::Token::new_symbol(token::TokenType::Asterisk),
        token::Token::new_symbol(token::TokenType::EOF),

    ]
)]
#[case(
    "let five = 5;".to_string(),
    vec![
        token::Token::new_symbol(token::TokenType::Let),
        token::Token::new_ident("five"),
        token::Token::new_symbol(token::TokenType::Assign),
        token::Token::new_number("5"),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::EOF),

    ]
)]
fn test_lexer(#[case] input: String, #[case] expected: Vec<token::Token>) {
    let mut l = Lexer::new(input);
    for expected_token in expected {
        let tok = l.next_token();
        assert_eq!(tok.token_type, expected_token.token_type, "expected: {:?}, got: {:?}", expected_token.token_type, tok.token_type);
        assert_eq!(tok.literal, expected_token.literal, "expected: {:?}, got: {:?}", expected_token.literal, tok.literal);
    }
}
