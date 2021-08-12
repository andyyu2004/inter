use crate::lex::{Token, TokenKind};
use crate::Result;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: I,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn accept(&mut self, kind: TokenKind) -> Option<Token> {
        self.expect(kind).ok()
    }

    pub fn expect(&mut self, kind: TokenKind) -> Result<Token> {
        match kind {
            TokenKind::Integer(_) => todo!(),
            TokenKind::Ident(_) => todo!(),
            TokenKind::OpenParen => todo!(),
            TokenKind::CloseParen => todo!(),
            TokenKind::Plus => todo!(),
            TokenKind::Minus => todo!(),
        }
    }
}
