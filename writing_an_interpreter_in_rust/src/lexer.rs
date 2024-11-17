use crate::token;
use rstest::rstest;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    line: usize,
    column: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            line: 1,
            column: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
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
        self.column += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }

    fn jump_ahead(&mut self, n: usize) {
        self.position += n;
        self.read_position += n;
        if self.position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.position).unwrap();
        }
    }

    fn try_lookahead(&self, literal: &str) -> bool {
        let length = literal.len();
        if self.position + length > self.input.len() {
            return false;
        }
        let lookahead = self.input[self.position..self.position + length].to_string();
        return lookahead == literal;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let line = self.line;
        let col = self.column;

        if let Some(mut tok) = self.try_multi_char_token() {
            tok.line = line;
            tok.column = col;
            return tok;
        }

        if let Some(mut tok) = self.try_single_char_token() {
            tok.line = line;
            tok.column = col;
            return tok;
        }

        if let Some(mut tok) = self.try_ident_or_keyword() {
            tok.line = line;
            tok.column = col;
            return tok;
        }

        if let Some(mut tok) = self.try_number() {
            tok.line = line;
            tok.column = col;
            return tok;
        }

        if let Some(mut tok) = self.try_eof() {
            tok.line = line;
            tok.column = col;
            return tok;
        }

        return token::Token::new_illegal(&self.ch.to_string());
    }

    fn try_multi_char_token(&mut self) -> Option<token::Token> {
        let token_types = token::lookup_multi_char_token(self.ch);
        let tok = token_types
            .iter()
            .filter_map(|token_type| {
                let tok = token::Token::new_symbol(token_type.clone());
                if self.try_lookahead(&tok.literal) {
                    return Some(tok);
                }
                return None;
            })
            .max_by_key(|tok| tok.literal.len());

        if let Some(tok) = tok {
            self.jump_ahead(tok.literal.len());
            return Some(tok);
        }
        return None;
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
    "let five = 5;
let ten = 10;".to_string(),
    vec![
        token::Token::new_symbol(token::TokenType::Let),
        token::Token::new_ident("five"),
        token::Token::new_symbol(token::TokenType::Assign),
        token::Token::new_number("5"),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::Let),
        token::Token::new_ident("ten"),
        token::Token::new_symbol(token::TokenType::Assign),
        token::Token::new_number("10"),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::EOF),
    ]
)]
#[case(
"let add = fn(x, y) {
    x + y;
};".to_string(),
vec![
			token::Token::new_symbol(token::TokenType::Let),
			token::Token::new_ident("add"),
			token::Token::new_symbol(token::TokenType::Assign),
			token::Token::new_symbol(token::TokenType::Function),
			token::Token::new_symbol(token::TokenType::LParen),
			token::Token::new_ident("x"),
			token::Token::new_symbol(token::TokenType::Comma),
			token::Token::new_ident("y"),
			token::Token::new_symbol(token::TokenType::RParen),
			token::Token::new_symbol(token::TokenType::LBrace),
            token::Token::new_ident("x"),
			token::Token::new_symbol(token::TokenType::Plus),
            token::Token::new_ident("y"),
			token::Token::new_symbol(token::TokenType::Semicolon),
			token::Token::new_symbol(token::TokenType::RBrace),
			token::Token::new_symbol(token::TokenType::Semicolon),
			token::Token::new_symbol(token::TokenType::EOF),
        ],
)]
#[case(
    "let result = add(five, ten);".to_string(),
    vec![
        token::Token::new_symbol(token::TokenType::Let),
        token::Token::new_ident("result"),
        token::Token::new_symbol(token::TokenType::Assign),
        token::Token::new_ident("add"),
        token::Token::new_symbol(token::TokenType::LParen),
        token::Token::new_ident("five"),
        token::Token::new_symbol(token::TokenType::Comma),
        token::Token::new_ident("ten"),
        token::Token::new_symbol(token::TokenType::RParen),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::EOF),
    ],
)]
#[case(
"!-/*5;
5 < 10 > 5;".to_string(),
vec![
			token::Token::new_symbol(token::TokenType::Bang),
			token::Token::new_symbol(token::TokenType::Minus),
			token::Token::new_symbol(token::TokenType::Slash),
			token::Token::new_symbol(token::TokenType::Asterisk),
			token::Token::new_number("5"),
			token::Token::new_symbol(token::TokenType::Semicolon),
			token::Token::new_number("5"),
			token::Token::new_symbol(token::TokenType::LT),
			token::Token::new_number("10"),
			token::Token::new_symbol(token::TokenType::GT),
			token::Token::new_number("5"),
			token::Token::new_symbol(token::TokenType::Semicolon),
			token::Token::new_symbol(token::TokenType::EOF),
        ],
)]
#[case(
"if (5 < 10) {
    return true;
} else {
    return false;
}".to_string(),
vec![
        token::Token::new_symbol(token::TokenType::If),
        token::Token::new_symbol(token::TokenType::LParen),
        token::Token::new_number("5"),
        token::Token::new_symbol(token::TokenType::LT),
        token::Token::new_number("10"),
        token::Token::new_symbol(token::TokenType::RParen),
        token::Token::new_symbol(token::TokenType::LBrace),
        token::Token::new_symbol(token::TokenType::Return),
        token::Token::new_symbol(token::TokenType::True),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::RBrace),
        token::Token::new_symbol(token::TokenType::Else),
        token::Token::new_symbol(token::TokenType::LBrace),
        token::Token::new_symbol(token::TokenType::Return),
        token::Token::new_symbol(token::TokenType::False),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::RBrace),
        token::Token::new_symbol(token::TokenType::EOF),
],
)]
#[case(
"10 == 10;
10 != 9;".to_string(),
vec![
        token::Token::new_number("10"),
        token::Token::new_symbol(token::TokenType::EQ),
        token::Token::new_number("10"),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_number("10"),
        token::Token::new_symbol(token::TokenType::NotEQ),
        token::Token::new_number("9"),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::EOF),
],
)]
#[case(
"x++; y--;".to_string(),
vec![
        token::Token::new_ident("x"),
        token::Token::new_symbol(token::TokenType::PlusPlus),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_ident("y"),
        token::Token::new_symbol(token::TokenType::MinusMinus),
        token::Token::new_symbol(token::TokenType::Semicolon),
        token::Token::new_symbol(token::TokenType::EOF),
],
)]
#[case(
"<= >=".to_string(),
vec![
        token::Token::new_symbol(token::TokenType::LTE),
        token::Token::new_symbol(token::TokenType::GTE),
        token::Token::new_symbol(token::TokenType::EOF),
],
)]
#[case(
"===".to_string(),
vec![
        token::Token::new_symbol(token::TokenType::STRONGEQ),
        token::Token::new_symbol(token::TokenType::EOF),
],
)]
fn test_lexer(#[case] input: String, #[case] expected: Vec<token::Token>) {
    let mut l = Lexer::new(input);
    for expected_token in expected {
        let tok = l.next_token();
        assert_eq!(
            tok.token_type, expected_token.token_type,
            "expected: {:?}, got: {:?}",
            expected_token.token_type, tok.token_type
        );
        assert_eq!(
            tok.literal, expected_token.literal,
            "expected: {:?}, got: {:?}",
            expected_token.literal, tok.literal
        );
    }
}
