use std::fmt::Debug;

#[derive(PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
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
    PlusPlus,
    MinusMinus,

    LT,
    GT,
    LTE,
    GTE,

    EQ,
    STRONGEQ,
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

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self.to_string()))
    }
}

impl TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::ILLEGAL => "ILLEGAL".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::Ident(_) => "Identifier".to_string(),
            TokenType::Int(_) => "Integer".to_string(),
            TokenType::Assign => "=".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Bang => "!".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::Slash => "/".to_string(),
            TokenType::PlusPlus => "++".to_string(),
            TokenType::MinusMinus => "--".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::LTE => "<=".to_string(),
            TokenType::GTE => ">=".to_string(),
            TokenType::EQ => "==".to_string(),
            TokenType::STRONGEQ => "===".to_string(),
            TokenType::NotEQ => "!=".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::LParen => "(".to_string(),
            TokenType::RParen => ")".to_string(),
            TokenType::LBrace => "{".to_string(),
            TokenType::RBrace => "}".to_string(),
            TokenType::Function => "fn".to_string(),
            TokenType::Let => "let".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::False => "false".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::Return => "return".to_string(),
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
            line: 0,
            column: 0,
        }
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
            TokenType::Ident(_) => Token::new(TokenType::ILLEGAL, "".to_string()),
            TokenType::Int(_) => Token::new(TokenType::ILLEGAL, "".to_string()),
            TokenType::PlusPlus => Token::new(TokenType::PlusPlus, "++".to_string()),
            TokenType::MinusMinus => Token::new(TokenType::MinusMinus, "--".to_string()),
            TokenType::LTE => Token::new(TokenType::LTE, "<=".to_string()),
            TokenType::GTE => Token::new(TokenType::GTE, ">=".to_string()),
            TokenType::EQ => Token::new(TokenType::EQ, "==".to_string()),
            TokenType::STRONGEQ => Token::new(TokenType::STRONGEQ, "===".to_string()),
            TokenType::NotEQ => Token::new(TokenType::NotEQ, "!=".to_string()),
            TokenType::ILLEGAL => Token::new(TokenType::ILLEGAL, "".to_string()),
        }
    }

    pub fn new_illegal(literal: &str) -> Token {
        Token::new(TokenType::ILLEGAL, literal.to_string())
    }

    pub fn new_ident(literal: &str) -> Token {
        Token::new(TokenType::Ident(literal.to_string()), literal.to_string())
    }

    pub fn new_number(literal: &str) -> Token {
        Token::new(
            TokenType::Int(literal.parse().unwrap()),
            literal.to_string(),
        )
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

pub fn lookup_multi_char_token(c: char) -> Vec<TokenType> {
    match c {
        '=' => vec![TokenType::STRONGEQ, TokenType::EQ],
        '!' => vec![TokenType::NotEQ],
        '<' => vec![TokenType::LTE],
        '>' => vec![TokenType::GTE],
        '-' => vec![TokenType::MinusMinus],
        '+' => vec![TokenType::PlusPlus],
        _ => vec![],
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
