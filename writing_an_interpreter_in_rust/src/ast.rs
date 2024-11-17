use std::fmt::Debug;

use crate::token;

pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

impl Debug for ProgramNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for statement in &self.statements {
            result.push_str(&format!("{:?}", statement));
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Identifier(IdentfierExpr),
    Literal(LiteralExpr),
}

#[derive(Debug)]
pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
}

#[derive(Debug)]
pub struct IdentfierExpr {
    pub token: token::Token,
    pub value: String,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub token: token::Token,
    pub value: LiteralEnum
}

#[derive(Debug)]
pub enum LiteralEnum {
    IntLiteral(i64),
    StringLiteral(String),
    Boolean(bool),
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: token::Token,
    pub name: IdentfierExpr,
    pub value: ExpressionNode,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub value: ExpressionNode,
}
