use crate::lex::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // here just for pretty printing
    Group(Box<Expr>),
    Bin(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Int(i64),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl From<Token> for BinOp {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Plus => Self::Plus,
            TokenKind::Minus => Self::Minus,
            TokenKind::Star => Self::Multiply,
            TokenKind::Slash => Self::Divide,
            _ => panic!("invalid binop `{:?}`", token),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UnaryOp {
    Not,
    Negate,
}

impl From<Token> for UnaryOp {
    fn from(token: Token) -> Self {
        match token.kind {
            TokenKind::Minus => Self::Negate,
            TokenKind::Bang => Self::Not,
            _ => panic!("invalid unary op `{:?}`", token),
        }
    }
}
