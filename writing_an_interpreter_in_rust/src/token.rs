#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String),
    Int(i64),
    
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    EQ,
    NotEQ,
     
    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token { token_type, literal }
    }
    pub fn new_symbol(token_type: TokenType) -> Token {
        match token_type {
            TokenType::Assign => Token::new(token_type, "=".to_string()),
            TokenType::Plus => Token::new(token_type, "+".to_string()),
            TokenType::Minus => Token::new(token_type, "-".to_string()),
            TokenType::Bang => Token::new(token_type, "!".to_string()),
            TokenType::Asterisk => Token::new(token_type, "*".to_string()),
            TokenType::Slash => Token::new(token_type, "/".to_string()),
            TokenType::LT => Token::new(token_type, "<".to_string()),
            TokenType::GT => Token::new(token_type, ">".to_string()),
            TokenType::Comma => Token::new(token_type, ",".to_string()),
            TokenType::Semicolon => Token::new(token_type, ";".to_string()),
            TokenType::LParen => Token::new(token_type, "(".to_string()),
            TokenType::RParen => Token::new(token_type, ")".to_string()),
            TokenType::LBrace => Token::new(token_type, "{".to_string()),
            TokenType::RBrace => Token::new(token_type, "}".to_string()),
            TokenType::Function => Token::new(token_type, "fn".to_string()),
            TokenType::Let => Token::new(token_type, "let".to_string()),
            TokenType::True => Token::new(token_type, "true".to_string()),
            TokenType::False => Token::new(token_type, "false".to_string()),
            TokenType::If => Token::new(token_type, "if".to_string()),
            TokenType::Else => Token::new(token_type, "else".to_string()),
            TokenType::Return => Token::new(token_type, "return".to_string()),
            TokenType::EOF => Token::new(token_type, "".to_string()),
            _ => panic!("Invalid token type"),
        }
    }

    pub fn new_illegal(literal: &str) -> Token {
        Token::new(TokenType::Illegal, literal.to_string())
    }

    pub fn new_ident(literal: &str) -> Token {
        Token::new(TokenType::Ident(literal.to_string()), literal.to_string())
    }

    pub fn new_number(literal: &str) -> Token {
        Token::new(TokenType::Int(literal.parse().unwrap()), literal.to_string())
    }
}


pub fn lookup_single_char_token(c: char) -> Option<TokenType> {
    match c {
        '=' => Some(TokenType::Assign),
        '+' => Some(TokenType::Plus),
        '-' => Some(TokenType::Minus),
        '!' => Some(TokenType::Bang),
        '*' => Some(TokenType::Asterisk),
        '/' => Some(TokenType::Slash),
        '<' => Some(TokenType::LT),
        '>' => Some(TokenType::GT),
        ',' => Some(TokenType::Comma),
        ';' => Some(TokenType::Semicolon),
        '(' => Some(TokenType::LParen),
        ')' => Some(TokenType::RParen),
        '{' => Some(TokenType::LBrace),
        '}' => Some(TokenType::RBrace),
        _ => None,
    }
}

pub fn lookup_keyword(ident: &str) -> Option<TokenType> {
    match ident {
        "fn" => Some(TokenType::Function),
        "let" => Some(TokenType::Let),
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "return" => Some(TokenType::Return),
        _ => None,
    }
}

pub fn possible_multi_char_token(c: char) -> bool {
    match c {
        '=' | '!' | '<' | '>' => true,
        _ => false,
    }
}
