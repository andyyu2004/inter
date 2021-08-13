use crate::ast::{BinOp, Expr, UnaryOp};
use crate::lex::{Symbol, Token, TokenKind};
use crate::Result;
use std::iter::Peekable;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn parse_expr(&mut self) -> Result<Box<Expr>> {
        self.parse_term()
    }

    pub fn parse_term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.parse_factor()?;
        const TERM_OP_TOKENS: [TokenKind; 2] = [TokenKind::Plus, TokenKind::Minus];
        while TERM_OP_TOKENS.contains(&self.peek_token()?.kind) {
            let op = BinOp::from(self.next_token()?);
            let rhs = self.parse_factor()?;
            expr = Box::new(Expr::Bin(expr, op, rhs))
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.parse_unary()?;
        const FACTOR_OP_TOKENS: [TokenKind; 2] = [TokenKind::Star, TokenKind::Slash];
        while FACTOR_OP_TOKENS.contains(&self.peek_token()?.kind) {
            let op = BinOp::from(self.next_token()?);
            let rhs = self.parse_unary()?;
            expr = Box::new(Expr::Bin(expr, op, rhs))
        }
        return Ok(expr);
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>> {
        const UNARY_OPS: [TokenKind; 2] = [TokenKind::Minus, TokenKind::Bang];
        if UNARY_OPS.contains(&self.peek_token()?.kind) {
            let op = UnaryOp::from(self.next_token()?);
            let mut expr = self.parse_unary()?;
            Ok(Box::new(Expr::Unary(op, expr)))
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> Result<Box<Expr>> {
        let expr = if self.accept(TokenKind::OpenParen).is_some() {
            let inner_expr = self.parse_expr()?;
            self.expect(TokenKind::CloseParen)?;
            return Ok(inner_expr);
        } else if let Some(token) = self.accept(TokenKind::Integer) {
            Expr::Int(token.lexeme.parse().unwrap())
        } else {
            todo!("{:?}", self.next_token())
        };
        Ok(Box::new(expr))
    }
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self { tokens: tokens.peekable() }
    }

    fn peek_token(&mut self) -> Result<Token> {
        self.tokens.peek().cloned().ok_or_else(|| anyhow!("unexpected eof"))
    }

    fn next_token(&mut self) -> Result<Token> {
        self.tokens.next().ok_or_else(|| anyhow!("unexpected eof"))
    }

    pub fn accept(&mut self, kind: TokenKind) -> Option<Token> {
        self.expect(kind).ok()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token> {
        let token = self.peek_token()?;
        if token.kind == kind {
            // only go next when ok for `accept` to be correct
            self.next_token();
            Ok(token)
        } else {
            bail!("expected token `{:?}` found token `{:?}`", kind, token.kind)
        }
    }
}

#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! parse_expr {
    ($src:expr) => {{
        let tokens = $crate::lex::lex($src);
        $crate::parse::Parser::new(tokens).parse_expr().unwrap()
    }};
}
