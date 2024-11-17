use crate::ast;
use crate::lexer;
use crate::token;
use rstest::rstest;

pub struct Parser {
    l: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: lexer::Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: token::Token::new(token::TokenType::EOF, "".to_string()),
            peek_token: token::Token::new(token::TokenType::EOF, "".to_string()),
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> ast::ProgramNode {
        let mut program_node = ast::ProgramNode {
            statements: Vec::new(),
        };
        while self.cur_token.token_type != token::TokenType::EOF {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program_node.statements.push(stmt);
            }
            self.next_token();
        }
        return program_node;
    }

    pub fn parse_statement(&mut self) -> Option<ast::StatementNode> {
        match self.cur_token.token_type {
            token::TokenType::Let => self.parse_let_statement(),
            token::TokenType::Return => self.parse_return_statement(),
            _ => None
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<ast::StatementNode> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::TokenType::Ident("".to_string())) {
            return None;
        }
        let name = ast::IdentfierExpr {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(token::TokenType::Assign) {
            return None;
        }
        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !self.cur_token_is(&token::TokenType::Semicolon) {
            self.next_token();
            if self.cur_token_is(&token::TokenType::EOF) {
                self.expect_peek(token::TokenType::Semicolon);
                return None;
            }
        }
        return Some(ast::StatementNode::Let(ast::LetStatement {
            token,
            name,
            value: ast::ExpressionNode::Literal(ast::LiteralExpr {
                token: token::Token::new(token::TokenType::Int(0), "0".to_string()),
                value: ast::LiteralEnum::IntLiteral(0),
            })
        }));
    }

    pub fn parse_return_statement(&mut self) -> Option<ast::StatementNode> {
        let token = self.cur_token.clone();
        self.next_token();
        // TODO: We're skipping the expressions until we
        // encounter a semicolon
        while !(self.cur_token_is(&token::TokenType::Semicolon) || self.cur_token_is(&token::TokenType::EOF)) {
            self.next_token();
        }
        return Some(ast::StatementNode::Return(ast::ReturnStatement {
            token,
            value: ast::ExpressionNode::Literal(ast::LiteralExpr {
                token: token::Token::new(token::TokenType::Int(0), "0".to_string()),
                value: ast::LiteralEnum::IntLiteral(0),
            })
        }));
    }

    pub fn cur_token_is(&self, t: &token::TokenType) -> bool {
        std::mem::discriminant(&self.cur_token.token_type) == std::mem::discriminant(t)
    }

    pub fn peek_token_is(&self, t: &token::TokenType) -> bool {
        std::mem::discriminant(&self.peek_token.token_type) == std::mem::discriminant(t)
    }

    pub fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(&t);
            return false;
        }
    }

    pub fn peek_error(&mut self, t: &token::TokenType) {
        let msg = format!("expected next token to be {:?}, got {:?} instead. Line {:?}, col {:?}", t, self.peek_token.literal, self.peek_token.line, self.peek_token.column);
        self.errors.push(msg);
    }

    pub fn has_errors(&self) -> bool {
        return self.errors.len() > 0;
    }

    pub fn errors(&self) -> Vec<String> {
        return self.errors.clone();
    }
}

#[rstest]
#[case("let x = 5;", "x", true)]
#[case("let y = 10;", "y", true)]
#[case("let foobar = 129243;", "foobar", true)]
#[case("let = 10;", "", false)]
fn test_parse_let(#[case] input: &str, #[case] expected_identifier: &str, #[case] valid: bool) {
    let l = lexer::Lexer::new(input.to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    assert_eq!(p.errors.len() == 0, valid);
    if !valid {
        return;
    }
    assert_eq!(program.statements.len(), 1);
    let stmt = program.statements.first().unwrap();
    match stmt {
        ast::StatementNode::Let(stmt) => {
            assert_eq!(stmt.name.value, expected_identifier);
            assert_eq!(stmt.name.token.literal, expected_identifier);
        },
        _ => panic!("Expected Let statement"),
    }
}
